use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub enum UiPanelButton {
    Close,
}

impl UiPanelButton {
    pub fn spawn(self, parent: &mut ChildBuilder, size: Val) {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(size, size),
                    ..default()
                },
                color: self.color(&Interaction::None).into(),
                ..default()
            })
            .insert(self);
    }

    pub fn color(&self, interaction: &Interaction) -> Color {
        match self {
            UiPanelButton::Close => match *interaction {
                Interaction::Clicked => Color::rgb(1.0, 0.4, 0.4),
                Interaction::Hovered => Color::rgb(0.7, 0.3, 0.3),
                Interaction::None => Color::rgb(0.8, 0.4, 0.4),
            },
        }
    }
}
