mod buttons;
mod util;

use bevy::{prelude::*, ui::FocusPolicy};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use buttons::UiPanelButton;
use util::climb_parents;

const PANEL_TITLEBAR_COLOR: Color = Color::rgb(0.3, 0.3, 0.6);
const PANEL_BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.8);

pub struct UiPanelsPlugin;

impl Plugin for UiPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<UiPanel>()
            .add_startup_system(setup)
            .add_system(panel_parenting)
            .add_system(panel_grabbing_and_dropping)
            .add_system(panel_dragging)
            .add_system(panel_updating)
            .add_system(panel_button_interaction);
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
    can_close: bool,
    panel_type: UiPanelType,
    drag_state: Option<UiPanelDragState>,
}

#[derive(Inspectable)]
pub struct UiPanelDragState {
    mouse_offset: Vec2,
}

#[derive(Component)]
pub struct UiPanelTitlebar;

#[derive(Component)]
pub struct UiPanelContent;

#[derive(Inspectable)]
pub enum UiPanelType {
    Window { position: Vec2 },
}

pub fn spawn_ui_panel(
    commands: &mut Commands,
    font: Handle<Font>,
    title: String,
    position: Vec2,
    size: Vec2,
    can_close: bool,
) -> Entity {
    let content_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                margin: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(2.0),
                },
                ..default()
            },
            color: PANEL_BACKGROUND_COLOR.into(),
            ..default()
        })
        .id();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::default(),
                size: Size::new(Val::Px(size.x), Val::Px(size.y)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            color: PANEL_TITLEBAR_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            spawn_ui_panel_titlebar(parent, font, title.clone(), can_close);
        })
        .add_child(content_entity)
        .insert(UiPanel {
            title,
            can_close,
            panel_type: UiPanelType::Window { position },
            drag_state: None,
        });

    content_entity
}

fn spawn_ui_panel_titlebar(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    title: String,
    can_close: bool,
) -> Entity {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(24.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            color: PANEL_TITLEBAR_COLOR.into(),
            ..default()
        })
        .insert(UiPanelTitlebar)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        title,
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            if can_close {
                UiPanelButton::Close.spawn(parent, Val::Px(16.0));
            }
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
    mut panel_parent_children_query: Query<&mut Children, With<UiPanelParent>>,
    mut panel_query: Query<(Entity, &mut UiPanel)>,
) {
    if let Some(window) = windows.get_primary() {
        if let Some(cursor_position) = window.cursor_position() {
            let cursor_position = Vec2::new(cursor_position.x, window.height() - cursor_position.y);

            if let Ok(mut parent_children) = panel_parent_children_query.get_single_mut() {
                for (interaction, titlebar_parent) in &interaction_query {
                    if let Ok((panel_entity, mut panel)) =
                        panel_query.get_mut(titlebar_parent.get())
                    {
                        if parent_children.contains(&panel_entity) {
                            match panel.panel_type {
                                UiPanelType::Window { position } => match *interaction {
                                    Interaction::Clicked => {
                                        let mouse_offset = position - cursor_position;
                                        panel.drag_state = Some(UiPanelDragState { mouse_offset });

                                        let mut new_parent_children_vec = parent_children
                                            .iter()
                                            .copied()
                                            .filter(|entity| *entity != panel_entity)
                                            .collect::<Vec<Entity>>();
                                        new_parent_children_vec.push(panel_entity);

                                        *parent_children = Children::with(&new_parent_children_vec);
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
                    UiPanelType::Window { ref mut position } => {
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
            UiPanelType::Window { position } => {
                style.position.left = Val::Px(position.x);
                style.position.top = Val::Px(position.y);
            }
        }
    }
}

fn panel_button_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &UiPanelButton, &mut UiColor),
        Changed<Interaction>,
    >,
    parent_query: Query<&Parent>,
    panel_query: Query<Entity, With<UiPanel>>,
) {
    for (panel_button_entity, interaction, panel_button, mut color) in &mut interaction_query {
        *color = panel_button.color(interaction).into();

        if let Interaction::Clicked = *interaction {
            match panel_button {
                UiPanelButton::Close => {
                    if let Some(panel_entity) =
                        climb_parents(&parent_query, &panel_query, panel_button_entity)
                    {
                        commands.entity(panel_entity).despawn_recursive();
                    }
                }
            }
        }
    }
}
