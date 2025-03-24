use crate::prelude::*;

use super::{
    ball::Ball,
    paddles::{Height, PaddleType},
    Gameplay,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (paddle_input)
            .in_set(UpdateSet::RecordInput)
            .run_if(in_state(Gameplay::InPlay)),
    );
}
fn paddle_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Position, &PaddleType, &Height), Without<Ball>>,
    ball: Query<&Position, With<Ball>>,
) {
    query
        .iter_mut()
        .for_each(|(mut postition, paddle, height)| match paddle {
            PaddleType::Ai => {
                let ball = get_single!(ball);
                let a_to_b = ball.0 - postition.0;
                let new_pos = postition.0.y + a_to_b.y.signum() * 0.;
                postition.0.y =
                    new_pos.clamp(0. + height.h / 2., GAME_H - height.h / 2.)
            }
            PaddleType::Player => {
                let up: f32 = if kb_input.any_pressed([
                    KeyCode::ArrowUp,
                    KeyCode::KeyK,
                    KeyCode::KeyW,
                ]) {
                    1.
                } else {
                    0.
                };
                let down: f32 = if kb_input.any_pressed([
                    KeyCode::ArrowDown,
                    KeyCode::KeyJ,
                    KeyCode::KeyS,
                ]) {
                    1.
                } else {
                    0.
                };
                let new_pos = postition.0.y + (up - down) * 2.;
                postition.0.y =
                    new_pos.clamp(0. + height.h / 2., GAME_H - height.h / 2.)
            }
        });
}
