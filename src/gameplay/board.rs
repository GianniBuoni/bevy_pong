use crate::prelude::*;

use super::{
    ball::SpawnBall,
    bg::SpawnBg,
    gutters::{GuttersSide, SpawnGutter},
    paddles::{PaddleType, SpawnPaddle},
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

    SpawnBall.apply(world);
}

fn init_game(
    mut query: Query<(&mut Transform, &Position, &ZIndex), Added<Position>>,
) {
    for (mut transform, pos, z_index) in &mut query {
        transform.translation = pos.0.extend(z_index.0 as f32);
    }
}

#[derive(Component)]
pub(super) struct Position(pub Vec2);
