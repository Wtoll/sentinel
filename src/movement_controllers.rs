use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_rapier2d::prelude::*;

/// Plugin for configuring the movement controller systems.
pub struct MovementControllerPlugin;

impl bevy::app::Plugin for MovementControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ProcessMovementControllers, process_movement_controllers);
    }
}

/// Schedule label for internally processing the data written to the player controller.
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessMovementControllers;

#[derive(Component, Default)]
pub struct MovementController {
    pub terminal_velocity: f32,
    pub lateral_bearing: f32,
}

fn process_movement_controllers(
    mut commands: Commands,
    mut rapier_access: Query<(
        &mut RapierContextSimulation,
        &mut RapierContextColliders,
        &mut RapierRigidBodySet
    )>,
    mut players: Query<(
        Entity,
        &RapierContextEntityLink,
        &mut MovementController,
        Option<&mut Transform>,
    )>
) {
    // Iterate through all of the entities with rapier information and a movement controller
    for (
        entity,
        &RapierContextEntityLink(sub_entity),
        mut controller,
        transform
    ) in players.iter_mut() {

        // Get access to the entity's rapier physics information through its sub-entity
        if let Ok((mut simulation, mut colliders, mut rigid_bodies)) = rapier_access.get_mut(sub_entity) {

            let final_translation = Vec3::X * controller.lateral_bearing * 0.05;

            if let Some(mut transform) = transform {
                transform.translation += final_translation;
            } else {
                commands.entity(entity).insert(Transform::from_translation(final_translation));
            }

        }

    }
}