use bevy::prelude::*;

pub struct UiPanelsPlugin;

impl Plugin for UiPanelsPlugin {
    fn build(&self, _app: &mut App) {
        //app.add_startup_system(setup);
    }
}

#[derive(Bundle)]
pub struct UiPanelBundle {
    #[bundle]
    node_bundle: NodeBundle,
    panel: UiPanel,
}

#[derive(Component)]
pub struct UiPanel {
    title: String,
    panel_type: UiPanelType,
}

#[derive(Component)]
pub struct UiPanelTitlebar;

pub enum UiPanelType {
    Window { titlebar: Entity },
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
            titlebar: titlebar_entity.unwrap(),
        },
    });

    panel.id()
}

fn spawn_ui_panel_titlebar(parent: &mut ChildBuilder, font: Handle<Font>, title: String) -> Entity {
    parent
        .spawn_bundle(NodeBundle {
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
