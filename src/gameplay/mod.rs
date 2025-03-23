use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::board::spawn_board;
}

mod bg;
mod board;
mod paddles;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(board::plugin);
}
