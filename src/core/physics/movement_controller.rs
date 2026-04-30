use bevy::prelude::*;

/// An interface for providing an entity a degree of control over its movement
/// within the game world
#[derive(Component, Default)]
pub struct MovementController {
    /// The maximum velocity the entity is capable of generating moving at
    pub maximum_velocity: f32,
    /// The desired movement magnitude and direction along the lateral axis
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
        controller,
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