use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_ui_panels::{spawn_ui_panel, UiPanelsPlugin};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ExampleButton;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Bevy UI Panels - Testing"),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(UiPanelsPlugin)
        .add_startup_system(setup)
        .add_system(button_system)
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
        Vec2::new(200.0, 150.0),
        true,
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
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(ExampleButton)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Button",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    });

    let panel_entity_2 = spawn_ui_panel(
        &mut commands,
        font.clone(),
        String::from("Test Panel #2"),
        Vec2::new(50.0, 15.0),
        Vec2::new(150.0, 300.0),
        false,
    );

    commands.entity(panel_entity_2).with_children(|parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(96.0), Val::Px(32.0)),
                    margin: UiRect::all(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(ExampleButton)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Button #2",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<ExampleButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
