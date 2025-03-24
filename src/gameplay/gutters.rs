use crate::prelude::*;

pub(super) fn spawn_gutter(
    In(config): In<SpawnGutter>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let (w, h) = (GAME_W, 10.);
    let mesh = meshes.add(Rectangle::new(w, h));
    let y = match config.side {
        GuttersSide::Top => config.y + h / 2.,
        GuttersSide::Bottom => config.y - h / 2.,
    };

    commands.spawn((
        Name::new("Gutter"),
        Gutter,
        Position(Vec2::new(GAME_W / 2., y)),
        Mesh2d(mesh.clone()),
        RigidBody::Static,
        Collider::rectangle(w, h),
        Friction::new(0.),
        Restitution::new(1.),
        ZIndex(0),
    ));
}

#[derive(Component)]
pub(super) struct Gutter;

pub(super) struct SpawnGutter {
    pub y: f32,
    pub side: GuttersSide,
}

pub(super) enum GuttersSide {
    Top,
    Bottom,
}

impl Command for SpawnGutter {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_gutter, self);
    }
}
