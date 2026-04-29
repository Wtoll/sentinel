use bevy::prelude::*;

use crate::{core::{Player, input::{GameInput, InputSource}, physics::MovementController}, draft::behavior::scheduling::BrainSystemSet};

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            update_player_brains.in_set(BrainSystemSet))
            .configure_sets(Update, scheduling::AfterBrainSystemSet
                .after(BrainSystemSet));
    }
}

pub mod scheduling {
    use bevy::prelude::*;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct BrainSystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AfterBrainSystemSet;
}








fn update_player_brains(
    game_inputs: Query<&GameInput>,
    query: Query<(&InputSource, &mut MovementController), With<Player>>
) {
    for (input_source, mut movement_controller) in query {
        if let Ok(game_input) = game_inputs.get(input_source.0) {
            movement_controller.lateral_bearing = game_input.lateral_bearing;
        }
    }
}