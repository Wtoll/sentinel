use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MovementController {
    pub terminal_velocity: f32,
    pub lateral_bearing: f32,
}

pub(super) fn update_movement_controllers(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut MovementController,
        Option<&mut Transform>,
    )>
) {
    // Iterate through all of the entities with a movement controller
    for (
        entity,
        mut controller,
        transform
    ) in query.iter_mut() {

        let final_translation = Vec3::X * controller.lateral_bearing * 0.05;

        if let Some(mut transform) = transform {
            transform.translation += final_translation;
        } else {
            commands.entity(entity).insert(Transform::from_translation(final_translation));
        }
    }
}