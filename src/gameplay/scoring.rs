use crate::prelude::*;

use super::paddles::PaddleType;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Scores::default());
}

#[derive(Resource, Default)]
pub struct Scores {
    pub player: u32,
    pub ai: u32,
}

pub(super) struct RecordScore {
    pub scorer: PaddleType,
}

fn record_score(In(config): In<RecordScore>, mut score: ResMut<Scores>) {
    match config.scorer {
        PaddleType::Player => score.player += 1,
        PaddleType::Ai => score.ai += 1,
    }
}

impl Command for RecordScore {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(record_score, self);
    }
}
