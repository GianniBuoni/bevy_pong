use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Screen;
}

mod game;
mod pause;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>()
        .add_plugins((game::plugin, pause::plugin));
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub(super) enum Screen {
    #[default]
    Game,
}
