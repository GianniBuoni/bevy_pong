use crate::prelude::*;

fn spawn_ball(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let radius: f32 = 5.;
    let shape = meshes.add(Circle::new(radius));
    let color = materials.add(Color::WHITE);

    commands.spawn((
        Name::new("Ball"),
        Ball,
        Mesh2d(shape.clone()),
        MeshMaterial2d(color.clone()),
        Position(Vec2::new(GAME_W / 2., GAME_H / 2.)),
        StateScoped(Screen::Game),
        LinearVelocity(Vec2::new(-150., -150.)),
        RigidBody::Dynamic,
        Collider::circle(radius),
        //Collider options
        Restitution::new(1.),
        Friction::new(0.),
        ColliderDensity(2.),
        ZIndex(1),
    ));
}

#[derive(Component)]
pub(super) struct Ball;

pub(super) struct SpawnBall;

impl Command for SpawnBall {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached(spawn_ball);
    }
}
