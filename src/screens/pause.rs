use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Gameplay::Paused), spawn_pause_screen);
}

fn spawn_pause_screen(mut commands: Commands) {
    commands
        .ui_root_col()
        .with_children(|children| {
            children.header("PAUSED");
        })
        .insert((
            StateScoped(Gameplay::Paused),
            BackgroundColor(Color::srgba(0., 0., 0., 0.75)),
            ZIndex(2),
        ));
}
