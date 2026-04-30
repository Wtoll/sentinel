//! Entity behaviors

use bevy::prelude::*;

use leafwing_input_manager::prelude::*;

use crate::{core::{Player, input::{GameAction, InputSource}, physics::MovementController}, draft::behavior::scheduling::BrainSystemSet};

/// A plugin for enabling the game's behavior capabilities
pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            update_player_brains.in_set(BrainSystemSet));
    }
}

/// System scheduling in reference to the behavior system
pub mod scheduling {
    use bevy::prelude::*;

    /// A system set for updating brains in the world
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct BrainSystemSet;
}

/// System for updating player brains
fn update_player_brains(
    action_states: Query<&ActionState<GameAction>>,
    query: Query<(&InputSource, &mut MovementController), With<Player>>
) {
    for (input_source, mut movement_controller) in query {
        if let Ok(action_state) = action_states.get(input_source.0) {
            movement_controller.lateral_bearing = action_state.clamped_value(&GameAction::MoveLateral);
        }
    }
}