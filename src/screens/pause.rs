use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Gameplay::Paused), spawn_pause_screen);
}

fn spawn_pause_screen(mut commands: Commands) {
    commands
        .ui_root_col()
        .with_children(|children| {
            children.header("PAUSED");
            children.button("Resume").observe(resume);
            children.button("Main Menu").observe(main_menu);
            children.button("Quit").observe(super::quit);
        })
        .insert((
            StateScoped(Gameplay::Paused),
            BackgroundColor(Color::srgba(0., 0., 0., 0.75)),
            ZIndex(2),
        ));
}

// BUTTON EVENTS
fn resume(_trigger: Trigger<OnClick>, mut commands: Commands) {
    commands.run_system_cached(toggle_game_state);
}

fn main_menu(
    _trigger: Trigger<OnClick>,
    mut next: ResMut<NextState<Screen>>,
    mut scores: ResMut<Scores>,
    mut commands: Commands,
) {
    commands.run_system_cached(toggle_game_state);
    next.set(Screen::Title);
    *scores = Scores::default();
}
