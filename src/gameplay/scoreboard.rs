use crate::prelude::*;

use super::scoring::Scores;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        update_scoreboard.run_if(resource_exists::<Scores>),
    );
}

fn spawn_scoreboard(mut commands: Commands) {
    // TODO dividing dashed line
    let mut root = commands.ui_root_row();

    root.with_children(|children| {
        add_score(children, PlayerScore);
        add_score(children, AiScore);
    });
}

fn update_scoreboard(
    mut ai_score: Query<&mut Text, With<AiScore>>,
    mut player_score: Query<&mut Text, (With<PlayerScore>, Without<AiScore>)>,
    score: Res<Scores>,
) {
    if score.is_changed() {
        let mut player_score = get_single_mut!(player_score);
        let mut ai_score = get_single_mut!(ai_score);
        player_score.0 = score.player.to_string();
        ai_score.0 = score.ai.to_string();
    }
}

// TODO load saved scores here?
pub(super) struct SpawnScoreboard;

impl Command for SpawnScoreboard {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached(spawn_scoreboard);
    }
}

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

fn add_score(builder: &mut ChildBuilder, score: impl Component) {
    let mut command = builder.spawn((
        Name::new("Score"),
        StateScoped(Screen::Game),
        Node {
            width: Val::Percent(24.),
            height: Val::Px(65.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));
    command.with_children(|children| {
        children.spawn((
            Name::new("Score Text"),
            score,
            Text::new("0"),
            TextFont::from_font_size(64.),
            TextColor::WHITE,
        ));
    });
}
