use crate::prelude::*;

use super::board::Position;

fn spawn_bg(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let shape = meshes.add(Rectangle::new(GAME_W, GAME_H));
    let color = materials.add(Color::BLACK);

    commands.spawn((
        Name::new("Background"),
        Position(Vec2::new(GAME_W / 2., GAME_H / 2.)),
        Mesh2d(shape.clone()),
        MeshMaterial2d(color.clone()),
        StateScoped(Screen::Game),
        ZIndex(0),
    ));
}

pub(super) struct SpawnBg;

impl Command for SpawnBg {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached(spawn_bg);
    }
}
