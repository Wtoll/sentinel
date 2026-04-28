use bevy::prelude::*;

use crate::{input::{GameInput, InputSource}, level::player::Player, movement_controllers::MovementController};

pub struct BrainsPlugin;

impl bevy::app::Plugin for BrainsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(StateTransition, update_player_brain);
    }
}

fn update_player_brain(
    game_inputs: Query<&GameInput>,
    query: Query<(&InputSource, &mut MovementController), With<Player>>
) {
    for (input_source, mut movement_controller) in query {
        if let Ok(game_input) = game_inputs.get(input_source.0) {
            movement_controller.lateral_bearing = game_input.lateral_bearing;
        }
    }
}