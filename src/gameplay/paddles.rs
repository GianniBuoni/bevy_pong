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
        PaddleType::Player => Color::srgb(0., 1., 0.),
        PaddleType::Ai => Color::srgb(1., 0., 0.),
    };
    let material = materials.add(ColorMaterial::from_color(color));

    commands.spawn((
        Name::new("Paddle"),
        Paddle,
        config.paddle_type,
        Position(Vec2::new(config.x, GAME_H / 2.)),
        RigidBody::Kinematic,
        Collider::rectangle(10., 60.),
        Mesh2d(shape.clone()),
        MeshMaterial2d(material.clone()),
        StateScoped(Screen::Game),
        ZIndex(1),
    ));
}

#[derive(Component)]
pub(super) struct Paddle;

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
