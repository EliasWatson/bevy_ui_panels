use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_ui_panels::{spawn_ui_panel, UiPanelsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(UiPanelsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Medium.ttf");

    let panel_entity = spawn_ui_panel(
        &mut commands,
        font.clone(),
        String::from("Test Panel"),
        Vec2::new(200.0, 50.0),
        Vec2::new(300.0, 500.0),
    );

    commands.entity(panel_entity).with_children(|parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(32.0)),
                    margin: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::rgb(0.4, 0.4, 0.4).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Button",
                    TextStyle {
                        font,
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    });
}
