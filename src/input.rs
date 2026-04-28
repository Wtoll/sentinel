use std::ops::{Deref, DerefMut};

use bevy::{
    ecs::schedule::ScheduleLabel,
    input::gamepad::{GamepadConnection, GamepadConnectionEvent},
    prelude::*
};

use crate::{level::player::Player, movement_controllers::MovementController};

/// Plugin for handling input
pub struct GameInputPlugin;

impl bevy::app::Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ProcessGameInput, (process_new_gamepads, process_gamepad_inputs).chain());
    }
}

/// Schedule label for processing game input
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessGameInput;

/// Component for storing a source of user input data
#[derive(Component)]
pub struct InputSource(pub Entity);

/// Component for storing user inputs after mapping
#[derive(Component, Default)]
pub struct GameInput {
    pub lateral_bearing: f32
}

impl From<&Gamepad> for GameInput {
    fn from(gamepad: &Gamepad) -> Self {
        let mut gamepad_bearing = gamepad.left_stick().x;

        // Manually establish a dead zone
        // See bevy Issue #24009
        if gamepad_bearing.abs() < 0.05 {
            gamepad_bearing = 0.0;
        }

        Self {
            lateral_bearing: gamepad_bearing
        }
    }
}

/// System for linking new gamepads to player controllers
fn process_new_gamepads(
    mut commands: Commands,
    mut connection_events: MessageReader<GamepadConnectionEvent>,
    mut gamepad_settings: Query<&mut GamepadSettings>,
    players: Query<(Entity, Option<&InputSource>), With<Player>>
) {
    for event in connection_events.read() {
        let GamepadConnectionEvent { gamepad, connection } = event;
        match connection {
            GamepadConnection::Connected {
                name: _,
                vendor_id: _,
                product_id: _
            } => {
                // Disable broken deadzones
                // See bevy Issue #24009
                if let Ok(mut settings) = gamepad_settings.get_mut(*gamepad) {
                    settings.default_axis_settings.set_deadzone_lowerbound(0.0);
                    settings.default_axis_settings.set_deadzone_upperbound(0.0);
                } else {
                    let mut settings = GamepadSettings::default();

                    settings.default_axis_settings.set_deadzone_lowerbound(0.0);
                    settings.default_axis_settings.set_deadzone_upperbound(0.0);

                    commands.entity(*gamepad).insert(settings);
                }

                // If no players are reading from this gamepad...
                if players.iter().all(|(_, source)| {
                    source.is_none_or(|source| {
                        source.0 != *gamepad
                    })
                }) {
                    // ...and there is a player without a source...
                    if let Some((player, _)) = players.iter()
                        .filter(|(_, source)| {
                            source.is_none()
                        })
                        .next() {
                            // ...give that player this gamepad as a source
                            commands.entity(player).insert(InputSource(*gamepad));
                        }
                }
            },
            GamepadConnection::Disconnected => {
                commands.entity(*gamepad).remove::<GameInput>();
            },
        }
    }
}

/// System for reading game inputs and remapping them
fn process_gamepad_inputs(
    mut commands: Commands,
    gamepads: Query<(Entity, &Gamepad, Option<&mut GameInput>)>
) {

    for (entity, gamepad, game_input) in gamepads {
        let new_game_input = GameInput::from(gamepad);

        if let Some(mut game_input) = game_input {
            *game_input = new_game_input;
        } else {
            commands.entity(entity).insert(new_game_input);
        }
    }
}