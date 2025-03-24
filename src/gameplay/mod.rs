use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::board::spawn_board;
}

mod ball;
mod bg;
mod board;
mod gutters;
mod input;
mod paddles;
mod pause;
mod reset;
mod scoring;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        ball::plugin,
        board::plugin,
        input::plugin,
        pause::plugin,
        reset::plugin,
        scoring::plugin,
    ));
}
