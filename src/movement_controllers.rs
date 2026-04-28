use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

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
    mut players: Query<(
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
    ) in players.iter_mut() {

        let final_translation = Vec3::X * controller.lateral_bearing * 0.05;

        if let Some(mut transform) = transform {
            transform.translation += final_translation;
        } else {
            commands.entity(entity).insert(Transform::from_translation(final_translation));
        }

    }
}