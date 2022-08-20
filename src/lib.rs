use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct UiPanelsPlugin;

impl Plugin for UiPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<UiPanel>()
            .add_system(panel_grabbing_and_dropping);
    }
}

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
    Window { titlebar: Option<Entity> },
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
            position: UiRect {
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                ..default()
            },
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

fn panel_grabbing_and_dropping(
    interaction_query: Query<
        (&Interaction, &Parent),
        (Changed<Interaction>, With<UiPanelTitlebar>),
    >,
    mut panel_query: Query<&mut UiPanel>,
) {
    for (interaction, titlebar_parent) in &interaction_query {
        if let Ok(mut panel) = panel_query.get_mut(titlebar_parent.get()) {
            match *interaction {
                Interaction::Clicked => {
                    panel.drag_state = Some(UiPanelDragState {
                        mouse_offset: Vec2::new(0.0, 0.0),
                    });
                }
                _ if panel.drag_state.is_some() => {
                    panel.drag_state = None;
                }
                _ => {}
            }
        }
    }
}
