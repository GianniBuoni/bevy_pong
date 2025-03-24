use crate::prelude::*;

fn spawn_paddle(
    In(config): In<SpawnPaddle>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (w, h) = (10., 60.);
    let padding: f32 = 10.;

    let shape = meshes.add(Rectangle::new(w, h));
    let color = match config.paddle_type {
        PaddleType::Player => Color::srgb(0., 1., 0.),
        PaddleType::Ai => Color::srgb(1., 0., 0.),
    };
    let x = match config.paddle_type {
        PaddleType::Player => config.x + padding,
        PaddleType::Ai => config.x - padding,
    };
    let material = materials.add(ColorMaterial::from_color(color));

    commands.spawn((
        Name::new("Paddle"),
        Paddle,
        config.paddle_type,
        Position(Vec2::new(x, GAME_H / 2.)),
        RigidBody::Kinematic,
        Collider::rectangle(w, h),
        Mesh2d(shape.clone()),
        MeshMaterial2d(material.clone()),
        StateScoped(Screen::Game),
        Friction::new(0.),
        Restitution::new(1.1),
        Height { h },
        ZIndex(1),
    ));
}

#[derive(Component)]
pub(super) struct Paddle;

#[derive(Component)]
pub(super) struct Height {
    pub h: f32,
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
