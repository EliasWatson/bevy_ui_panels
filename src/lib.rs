use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct UiPanelsPlugin;

impl Plugin for UiPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<UiPanel>()
            .add_startup_system(setup)
            .add_system(panel_parenting)
            .add_system(panel_grabbing_and_dropping)
            .add_system(panel_dragging)
            .add_system(panel_updating);
    }
}

#[derive(Component, Inspectable)]
struct UiPanelParent;

#[derive(Bundle)]
pub struct UiPanelBundle {
    #[bundle]
    node_bundle: NodeBundle,
    panel: UiPanel,
}

#[derive(Component, Inspectable)]
pub struct UiPanel {
    title: String,
    panel_type: UiPanelType,
    drag_state: Option<UiPanelDragState>,
}

#[derive(Inspectable)]
pub struct UiPanelDragState {
    mouse_offset: Vec2,
}

#[derive(Component)]
pub struct UiPanelTitlebar;

#[derive(Inspectable)]
pub enum UiPanelType {
    Window {
        position: Vec2,
        titlebar: Option<Entity>,
    },
}

pub fn spawn_ui_panel(
    commands: &mut Commands,
    font: Handle<Font>,
    title: String,
    position: Vec2,
    size: Vec2,
) -> Entity {
    let mut panel = commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::default(),
            size: Size::new(Val::Px(size.x), Val::Px(size.y)),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        color: Color::rgb(0.5, 0.5, 0.8).into(),
        ..default()
    });

    let mut titlebar_entity: Option<Entity> = None;
    panel.with_children(|parent| {
        titlebar_entity = Some(spawn_ui_panel_titlebar(parent, font, title.clone()));
    });

    panel.insert(UiPanel {
        title,
        panel_type: UiPanelType::Window {
            position,
            titlebar: titlebar_entity,
        },
        drag_state: None,
    });

    panel.id()
}

fn spawn_ui_panel_titlebar(parent: &mut ChildBuilder, font: Handle<Font>, title: String) -> Entity {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(24.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.3, 0.3, 0.6).into(),
            ..default()
        })
        .insert(UiPanelTitlebar)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                title,
                TextStyle {
                    font,
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        })
        .id()
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(UiPanelParent);
}

fn panel_parenting(
    mut commands: Commands,
    panel_parent_query: Query<Entity, With<UiPanelParent>>,
    panel_query: Query<Entity, (With<UiPanel>, Without<Parent>)>,
) {
    if panel_query.is_empty() {
        return;
    }

    if let Ok(panel_parent) = panel_parent_query.get_single() {
        let panel_entities = panel_query.iter().collect::<Vec<Entity>>();
        commands.entity(panel_parent).push_children(&panel_entities);
    }
}

fn panel_grabbing_and_dropping(
    windows: Res<Windows>,
    interaction_query: Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<UiPanelTitlebar>),
    >,
    mut panel_query: Query<&mut UiPanel>,
) {
    if let Some(window) = windows.get_primary() {
        if let Some(cursor_position) = window.cursor_position() {
            let cursor_position = Vec2::new(cursor_position.x, window.height() - cursor_position.y);

            for (interaction, titlebar_parent) in &interaction_query {
                if let Ok(mut panel) = panel_query.get_mut(titlebar_parent.get()) {
                    match panel.panel_type {
                        UiPanelType::Window {
                            position,
                            titlebar: _,
                        } => match *interaction {
                            Interaction::Clicked => {
                                let mouse_offset = position - cursor_position;
                                panel.drag_state = Some(UiPanelDragState { mouse_offset });
                            }
                            _ if panel.drag_state.is_some() => {
                                panel.drag_state = None;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }
}

fn panel_dragging(windows: Res<Windows>, mut panel_query: Query<&mut UiPanel>) {
    if let Some(window) = windows.get_primary() {
        if let Some(cursor_position) = window.cursor_position() {
            let cursor_position = Vec2::new(cursor_position.x, window.height() - cursor_position.y);

            for mut panel in &mut panel_query {
                let mouse_offset = match panel.drag_state {
                    Some(ref drag_state) => drag_state.mouse_offset,
                    None => continue,
                };

                match &mut panel.panel_type {
                    UiPanelType::Window {
                        ref mut position,
                        titlebar: _,
                    } => {
                        *position = cursor_position + mouse_offset;
                    }
                }
            }
        }
    }
}

fn panel_updating(mut panel_query: Query<(&UiPanel, &mut Style)>) {
    for (panel, mut style) in &mut panel_query {
        match panel.panel_type {
            UiPanelType::Window {
                position,
                titlebar: _,
            } => {
                style.position.left = Val::Px(position.x);
                style.position.top = Val::Px(position.y);
            }
        }
    }
}
