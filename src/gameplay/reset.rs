use crate::prelude::*;

use super::{
    ball::{Ball, SpawnBall},
    paddles::PaddleType,
    pause::Gameplay,
    scoring::RecordScore,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (check_for_reset)
            .in_set(UpdateSet::Update)
            .run_if(in_state(Gameplay::InPlay)),
    );
}

pub(super) fn reset_ball(
    trigger: Trigger<Reset>,
    position: Query<&Position, With<Ball>>,
    mut commands: Commands,
) {
    let position = get_single!(position).0.x;
    let direction = if position <= GAME_W / 2. {
        commands.queue(RecordScore {
            scorer: PaddleType::Ai,
        });
        1.
    } else {
        commands.queue(RecordScore {
            scorer: PaddleType::Player,
        });
        -1.
    };
    let ball = trigger.entity();
    commands.entity(ball).despawn_recursive();
    commands.queue(SpawnBall { direction });
}

fn check_for_reset(
    ball: Query<(Entity, &Position), With<Ball>>,
    mut commands: Commands,
) {
    let (ball, position) = get_single!(ball);

    if position.0.x <= 0. || position.0.x >= GAME_W {
        info!("[observer triggered] Reset!");
        commands.trigger_targets(Reset, ball);
    }
}

#[derive(Event)]
pub(super) struct Reset;
