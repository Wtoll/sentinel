use bevy::prelude::*;

/// An interface for providing an entity a degree of control over its movement
/// within the game world
#[derive(Component, Default)]
#[require(Transform)]
pub struct MovementController {
    /// The maximum velocity the entity is capable of generating moving at
    pub maximum_velocity: f32,
    /// The desired movement magnitude and direction along the lateral axis
    pub lateral_bearing: f32,
}

pub(super) fn update_movement_controllers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut MovementController,
        &mut Transform,
    )>
) {
    // Iterate through all of the entities with a movement controller
    for (
        entity,
        controller,
        mut transform
    ) in query.iter_mut() {

        let final_translation = (controller.lateral_bearing * 5.0) * time.delta_secs() * Vec3::X;

        transform.translation += final_translation;
        
    }
}