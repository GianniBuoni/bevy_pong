use crate::prelude::*;

use super::scoring::Scores;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        update_scoreboard.run_if(resource_exists::<Scores>),
    );
    app.init_resource::<DashAssets>();
}

fn spawn_scoreboard(mut commands: Commands) {
    for n in (3..GAME_H as i32).rev().step_by(17) {
        commands.queue(SpawnDashLine { y: n as f32 });
    }
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

#[derive(Resource, Asset, Reflect, Clone)]
struct DashAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

impl FromWorld for DashAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            mesh: assets.add(Rectangle::new(2., 10.).into()),
            material: assets.add(Color::WHITE.into()),
        }
    }
}

fn spawn_dashed_line(
    In(config): In<SpawnDashLine>,
    dash_asset: Res<DashAssets>,
    mut command: Commands,
) {
    command.spawn((
        Name::new("Midcourt Dash"),
        Position::new(Vec2::new(GAME_W / 2., config.y - 10.)),
        RigidBody::Static,
        Mesh2d(dash_asset.mesh.clone()),
        MeshMaterial2d(dash_asset.material.clone()),
        StateScoped(Screen::Game),
        ZIndex(1),
    ));
}

struct SpawnDashLine {
    y: f32,
}

impl Command for SpawnDashLine {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_dashed_line, self);
    }
}
