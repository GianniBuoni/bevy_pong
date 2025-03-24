use crate::prelude::*;

use super::reset::reset_ball;

fn spawn_ball(
    In(config): In<SpawnBall>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let radius: f32 = 5.;
    let shape = meshes.add(Circle::new(radius));
    let color = materials.add(Color::WHITE);
    let init_y: f32 = if rand::random_bool(0.5) { 1. } else { -1. };
    let init_vector = Vec2::new(150. * config.direction, 150. * init_y);

    commands
        .spawn((
            Name::new("Ball"),
            Ball,
            Mesh2d(shape.clone()),
            MeshMaterial2d(color.clone()),
            Position(Vec2::new(GAME_W / 2., GAME_H / 2.)),
            StateScoped(Screen::Game),
            LinearVelocity(init_vector),
            RigidBody::Dynamic,
            Collider::circle(radius),
            //Collider options
            Restitution::new(1.),
            Friction::new(0.),
            ColliderDensity(2.),
            ZIndex(1),
        ))
        .observe(reset_ball);
}

#[derive(Component)]
pub(super) struct Ball;

pub(super) struct SpawnBall {
    pub direction: f32,
}

impl Command for SpawnBall {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_ball, self);
    }
}
