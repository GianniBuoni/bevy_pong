use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::Screen;
}

mod game;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .enable_state_scoped_entities::<Screen>()
        .add_plugins(game::plugin);
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub(super) enum Screen {
    #[default]
    Game,
}
