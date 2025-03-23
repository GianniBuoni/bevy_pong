use crate::prelude::*;

use super::paddles::{PaddleType, SpawnPaddle};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_game.in_set(UpdateSet::Draw));
}

pub fn spawn_board(world: &mut World) {
    let padding: f32 = 10.;

    SpawnPaddle {
        x: -(GAME_W / 2.) + padding,
        paddle_type: PaddleType::Player,
    }
    .apply(world);

    SpawnPaddle {
        x: (GAME_W / 2.) - padding,
        paddle_type: PaddleType::Ai,
    }
    .apply(world);
}

fn draw_game(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, pos) in &mut query {
        transform.translation = pos.0.extend(0.);
    }
}

#[derive(Component)]
pub(super) struct Position(pub Vec2);
