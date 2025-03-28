use bevy::render::camera::ScalingMode::AutoMin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;
mod gameplay;
mod physics;
mod prelude;
mod screens;
mod theming;

pub struct AppPlugin;

impl bevy::app::Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                UpdateSet::Tickers,
                UpdateSet::RecordInput,
                UpdateSet::Update,
            )
                .chain(),
        );
        app.add_systems(Startup, spawn_camera);
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Pong with physics engine.".to_string(),
                        ..Default::default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins((
            gameplay::plugin,
            physics::plugin,
            screens::plugin,
            theming::plugin,
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: AutoMin {
                min_width: GAME_W,
                min_height: GAME_H,
            },
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(GAME_W / 2., GAME_H / 2., 0.),
    ));
}
