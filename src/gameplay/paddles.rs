use crate::prelude::*;

use super::board::Position;

fn spawn_paddle(
    In(config): In<SpawnPaddle>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(10., 60.));
    let color = match config.paddle_type {
        PaddleType::Player => Color::WHITE,
        PaddleType::Ai => Color::srgb(1., 0., 0.),
    };
    let material = materials.add(ColorMaterial::from_color(color));

    commands.spawn((
        config.paddle_type,
        Position(Vec2::new(config.x, 0.)),
        Mesh2d(shape.clone()),
        MeshMaterial2d(material.clone()),
    ));
}

#[derive(Component)]
pub(super) enum PaddleType {
    Player,
    Ai,
}

// passes variables to paddle spawns
pub(super) struct SpawnPaddle {
    pub x: f32,
    pub paddle_type: PaddleType,
}

impl Command for SpawnPaddle {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_paddle, self);
    }
}
