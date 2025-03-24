use bevy::input::common_conditions::input_just_pressed;

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

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((board::plugin, input::plugin))
        .init_state::<Gameplay>()
        .add_systems(
            Update,
            toggle_game_state
                .run_if(input_just_pressed(KeyCode::Escape))
                .in_set(UpdateSet::RecordInput),
        );
}

fn toggle_game_state(
    current: Res<State<Gameplay>>,
    mut next: ResMut<NextState<Gameplay>>,
) {
    println!("leaving: {:?}", current.get());
    match current.get() {
        Gameplay::InPlay => next.set(Gameplay::Paused),
        Gameplay::Paused => next.set(Gameplay::InPlay),
    }
}

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub(super) enum Gameplay {
    #[default]
    InPlay,
    Paused,
}
