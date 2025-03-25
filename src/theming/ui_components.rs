use bevy::color::palettes::tailwind::{GRAY_600, GRAY_900, GRAY_950};

use crate::prelude::*;

use super::interactions::InteractionStateColors;

pub trait UIContainer {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root_col(&mut self) -> EntityCommands;
    fn ui_root_row(&mut self) -> EntityCommands;
}

impl UIContainer for Commands<'_, '_> {
    fn ui_root_col(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
    fn ui_root_row(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                row_gap: Val::Px(10.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}

/// An extention trait to simplify spawning common ui components.
pub trait UIComponent {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;
    /// Spawn a header label with text.
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;
    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
}

// TODO Implement UI Compnent for Bevy' Commands and ChildBuilder
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}

impl<T: Spawn> UIComponent for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut command = self.spawn((
            Name::new("Button"),
            Button,
            Node {
                width: Val::Px(200.),
                height: Val::Px(64.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::Srgba(GRAY_900)),
            InteractionStateColors {
                none: Color::Srgba(GRAY_900),
                hovered: Color::Srgba(GRAY_600),
                pressed: Color::Srgba(GRAY_950),
            },
        ));
        command.with_children(|children| {
            ChildBuild::spawn(
                children,
                (
                    Name::new("Button Text"),
                    Text(text.into()),
                    TextFont::from_font_size(32.),
                    TextColor::WHITE,
                ),
            );
        });
        command
    }
    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn((
            Name::new("Label"),
            Text(text.into()),
            TextFont::from_font_size(24.),
            TextColor::WHITE,
            Node {
                width: Val::Px(500.),
                ..default()
            },
        ))
    }
    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut command = self.spawn((
            Name::new("Header"),
            Node {
                width: Val::Px(500.),
                height: Val::Px(72.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::Srgba(GRAY_900)),
        ));
        command.with_children(|children| {
            ChildBuild::spawn(
                children,
                (
                    Name::new("Header Text"),
                    Text(text.into()),
                    TextFont::from_font_size(64.),
                    TextColor::WHITE,
                ),
            );
        });
        command
    }
}
