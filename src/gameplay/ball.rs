use crate::prelude::*;

use super::board::Position;

fn spawn_ball(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let shape = meshes.add(Circle::new(5.));
    let color = materials.add(Color::WHITE);

    commands.spawn((
        Name::new("Ball"),
        Ball,
        Mesh2d(shape.clone()),
        MeshMaterial2d(color.clone()),
        Position(Vec2::new(GAME_W / 2., GAME_H / 2.)),
        StateScoped(Screen::Game),
        ZIndex(1),
    ));
}

#[derive(Component)]
struct Ball;

pub(super) struct SpawnBall;

impl Command for SpawnBall {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached(spawn_ball);
    }
}
