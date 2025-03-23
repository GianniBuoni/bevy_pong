use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Game), spawn_levels);
}

fn spawn_levels(mut commands: Commands) {
    commands.queue(spawn_board);
}
