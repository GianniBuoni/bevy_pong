use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .ui_root_col()
        .with_children(|children| {
            children.header("Pong!");
            children.button("Play?").observe(play_game);
            children.button("Quit").observe(super::quit);
        })
        .insert(StateScoped(Screen::Title));
}

fn play_game(_trigger: Trigger<OnClick>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::Game);
}
