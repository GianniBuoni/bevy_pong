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
            children.button("Quit").observe(quit);
        })
        .insert((
            StateScoped(Gameplay::Paused),
            BackgroundColor(Color::srgba(0., 0., 0., 0.75)),
            ZIndex(2),
        ));
}

// BUTTON EVENTS
fn resume(_trigger: Trigger<OnClick>, mut next: ResMut<NextState<Gameplay>>) {
    next.set(Gameplay::InPlay);
}

fn quit(_trigger: Trigger<OnClick>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
