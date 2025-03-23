use crate::prelude::*;

use super::{
    ball::SpawnBall,
    bg::SpawnBg,
    paddles::{PaddleType, SpawnPaddle},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_game.in_set(UpdateSet::Draw));
}

pub fn spawn_board(world: &mut World) {
    let padding: f32 = 10.;

    SpawnBg.apply(world);

    SpawnPaddle {
        x: 0. + padding,
        paddle_type: PaddleType::Player,
    }
    .apply(world);

    SpawnPaddle {
        x: GAME_W - padding,
        paddle_type: PaddleType::Ai,
    }
    .apply(world);

    SpawnBall.apply(world);
}

fn draw_game(mut query: Query<(&mut Transform, &Position, &ZIndex)>) {
    for (mut transform, pos, z_index) in &mut query {
        transform.translation = pos.0.extend(z_index.0 as f32);
    }
}

#[derive(Component)]
pub(super) struct Position(pub Vec2);
