use crate::prelude::*;

use super::{
    ball::SpawnBall,
    bg::SpawnBg,
    gutters::{GuttersSide, SpawnGutter},
    paddles::{PaddleType, SpawnPaddle},
    scoreboard::SpawnScoreboard,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, init_game);
}

pub fn spawn_board(world: &mut World) {
    SpawnBg.apply(world);

    SpawnPaddle {
        x: 0.,
        paddle_type: PaddleType::Player,
    }
    .apply(world);

    SpawnPaddle {
        x: GAME_W,
        paddle_type: PaddleType::Ai,
    }
    .apply(world);

    SpawnGutter {
        y: GAME_H,
        side: GuttersSide::Top,
    }
    .apply(world);

    SpawnGutter {
        y: 0.,
        side: GuttersSide::Bottom,
    }
    .apply(world);

    SpawnScoreboard.apply(world);

    let direction: f32 = if rand::random_bool(0.5) { 1. } else { -1. };
    SpawnBall { direction }.apply(world);
}

fn init_game(mut query: Query<(&mut Transform, &ZIndex), Added<ZIndex>>) {
    for (mut transform, z_index) in &mut query {
        transform.translation.z = z_index.0 as f32;
    }
}
