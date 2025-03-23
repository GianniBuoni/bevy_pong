use bevy::render::camera::ScalingMode::AutoMin;

use crate::prelude::*;
mod gameplay;
mod prelude;
mod screens;

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
        )
        .add_systems(Startup, spawn_camera)
        .add_plugins(
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
        )
        .add_plugins((gameplay::plugin, screens::plugin));
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
