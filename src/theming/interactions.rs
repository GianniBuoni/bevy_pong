use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionStateColors>();
    app.add_systems(Update, (trigger_onclick, apply_interaction_color));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionStateColors {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

#[derive(Event)]
pub struct OnClick;

fn trigger_onclick(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    interaction_query.iter().for_each(|(entity, interaction)| {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnClick, entity);
        }
    });
}

fn apply_interaction_color(
    mut interaction_query: Query<
        (&Interaction, &InteractionStateColors, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    interaction_query.iter_mut().for_each(
        |(interaction, palette, mut bg_color)| {
            *bg_color = match interaction {
                Interaction::None => palette.none,
                Interaction::Pressed => palette.pressed,
                Interaction::Hovered => palette.hovered,
            }
            .into();
        },
    );
}
