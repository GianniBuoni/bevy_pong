use crate::prelude::*;
use bevy::input::common_conditions::input_just_pressed;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Gameplay>();
    app.add_observer(pause_game_state);
    app.add_observer(unpause_game_state);
    app.enable_state_scoped_entities::<Gameplay>();
    app.add_systems(
        Update,
        toggle_game_state
            .run_if(input_just_pressed(KeyCode::Escape))
            .in_set(UpdateSet::RecordInput),
    );
}

pub fn toggle_game_state(
    current: Res<State<Gameplay>>,
    mut commands: Commands,
) {
    match current.get() {
        Gameplay::InPlay => {
            commands.trigger(PauseEvent);
        }
        Gameplay::Paused => {
            commands.trigger(UnpauseEvent);
        }
    }
}

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum Gameplay {
    #[default]
    InPlay,
    Paused,
}

// IN PLAY EVENTS
#[derive(Event)]
pub(super) struct UnpauseEvent;

fn unpause_game_state(
    _trigger: Trigger<UnpauseEvent>,
    mut next: ResMut<NextState<Gameplay>>,
) {
    info!("[game state changed] InPlay");
    next.set(Gameplay::InPlay)
}

// PAUSE EVENTS
#[derive(Event)]
pub(super) struct PauseEvent;

fn pause_game_state(
    _trigger: Trigger<PauseEvent>,
    mut next: ResMut<NextState<Gameplay>>,
) {
    info!("[game state changed] Pause");
    next.set(Gameplay::Paused);
}
