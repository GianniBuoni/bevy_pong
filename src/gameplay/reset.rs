use crate::prelude::*;

use super::{
    ball::{Ball, SpawnBall},
    Gameplay,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (trigger_reset)
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
    let direction = if position <= GAME_W / 2. { 1. } else { -1. };
    let ball = trigger.entity();
    commands.entity(ball).despawn_recursive();
    commands.queue(SpawnBall { direction });
}

fn trigger_reset(
    ball: Query<(Entity, &Position), With<Ball>>,
    mut commands: Commands,
) {
    let (ball, position) = get_single!(ball);

    if position.0.x <= 0. || position.0.x >= GAME_W {
        info!("Triggering reset!");
        commands.trigger_targets(Reset, ball);
    }
}

#[derive(Event)]
pub(super) struct Reset;
