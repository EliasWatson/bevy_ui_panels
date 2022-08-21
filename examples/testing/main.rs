use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_ui_panels::{spawn_ui_panel, UiPanelsPlugin};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
enum ExampleButton {
    SpawnPanel,
    DoNothing,
}

#[derive(Component)]
struct ExampleText;

#[derive(Debug)]
struct PanelSpawnCount(u32);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Bevy UI Panels - Testing"),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        .insert_resource(PanelSpawnCount(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(UiPanelsPlugin)
        .add_startup_system(setup)
        .add_system(button_system)
        .add_system(update_example_text)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Medium.ttf");

    let panel_entity = spawn_ui_panel(
        &mut commands,
        font.clone(),
        String::from("Test Panel"),
        Vec2::new(300.0, 50.0),
        Vec2::new(200.0, 150.0),
        false,
    );

    commands.entity(panel_entity).with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .with_children(|parent| {
                ExampleButton::SpawnPanel.spawn(parent, &font, String::from("Spawn Panel"));

                parent
                    .spawn_bundle(TextBundle::default())
                    .insert(ExampleText);
            });
    });

    let panel_entity_2 = spawn_ui_panel(
        &mut commands,
        font.clone(),
        String::from("Test Panel #2"),
        Vec2::new(50.0, 200.0),
        Vec2::new(150.0, 300.0),
        false,
    );

    commands.entity(panel_entity_2).with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::WrapReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .with_children(|parent| {
                for i in 0..16 {
                    ExampleButton::DoNothing.spawn(parent, &font, format!("{}", i));
                }
            });
    });
}

fn spawn_panel(commands: &mut Commands, font: &Handle<Font>, index: u32, time: f32) {
    let panel_entity = spawn_ui_panel(
        commands,
        font.clone(),
        format!("Spawn #{}", index),
        Vec2::new(640.0, 360.0),
        Vec2::new(250.0, 75.0),
        true,
    );

    commands.entity(panel_entity).with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    format!("This is panel #{}", index),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ));

                parent.spawn_bundle(TextBundle::from_section(
                    format!("It was spawned at {:.2} seconds", time),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                ));
            });
    });
}

fn button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_count: ResMut<PanelSpawnCount>,
    time: Res<Time>,
    mut interaction_query: Query<
        (&Interaction, &ExampleButton, &mut UiColor),
        Changed<Interaction>,
    >,
) {
    let font = asset_server.load("fonts/FiraSans-Medium.ttf");

    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                if let ExampleButton::SpawnPanel = button {
                    spawn_panel(
                        &mut commands,
                        &font,
                        spawn_count.0,
                        time.seconds_since_startup() as f32,
                    );
                    spawn_count.0 += 1;
                }
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

fn update_example_text(
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<ExampleText>>,
) {
    let font = asset_server.load("fonts/FiraSans-Medium.ttf");

    for mut text in &mut text_query {
        text.sections = vec![TextSection::new(
            format!("{:.1} seconds", time.seconds_since_startup()),
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::BLACK,
            },
        )];
    }
}

impl ExampleButton {
    fn spawn(self, parent: &mut ChildBuilder, font: &Handle<Font>, text: String) {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    margin: UiRect::all(Val::Px(4.0)),
                    padding: UiRect::all(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            })
            .insert(self);
    }
}
