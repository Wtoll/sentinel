//! Input Handling
//! 

use bevy::{input::gamepad::{GamepadConnection, GamepadConnectionEvent}, prelude::*};

use leafwing_input_manager::prelude::*;

use crate::core::{Player, input::scheduling::InputSystemSet};

/// Plugin to enable the game's input processing systems
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_keyboard_entity)
            .add_systems(Update, (
                configure_new_gamepads,
                attach_input_maps,
                attach_gamepads_to_players,
                attach_players_to_devices)
                    .in_set(InputSystemSet));
    }
}

/// System scheduling in reference to the input processing system
pub mod scheduling {
    use bevy::prelude::*;

    /// A system set for handling input processing
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct InputSystemSet;
}

/// Marker component for the virtual keyboard entity
#[derive(Component)]
pub struct Keyboard;

/// An action that may be performed by a player in the world of the game
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameAction {
    /// Pause the game
    Pause,
    /// Primary interact
    PrimaryInteract,
    /// Lateral movement
    #[actionlike(Axis)]
    MoveLateral,
    /// Jump action
    Jump,
}

impl GameAction {
    fn default_gamepad_input_map() -> InputMap<Self> {
        InputMap::default()
            .with(Self::Pause, GamepadButton::Select)
            .with(Self::PrimaryInteract, GamepadButton::East)
            .with_axis(Self::MoveLateral, GamepadAxis::LeftStickX)
            .with(Self::Jump, GamepadButton::South)
    }

    fn default_keyboard_input_map() -> InputMap<Self> {
        InputMap::default()
            .with(Self::Pause, KeyCode::Escape)
            .with(Self::PrimaryInteract, KeyCode::KeyE)
            .with_axis(Self::MoveLateral, VirtualAxis::new(KeyCode::KeyA, KeyCode::KeyD))
            .with(Self::Jump, KeyCode::KeyW)
    }
}

/// Component for a dependant of user input data
#[derive(Component)]
#[relationship(relationship_target = InputDependants)]
pub struct InputSource(pub Entity);

/// Component for a source of user input data
#[derive(Component)]
#[relationship_target(relationship = InputSource)]
pub struct InputDependants(Vec<Entity>);

/// System for creating the virtual keyboard entity
fn create_keyboard_entity(
    mut commands: Commands
) {
    commands.spawn((
        Name::new("Keyboard"),
        Keyboard,
        GameAction::default_keyboard_input_map()
    ));
}

/// System for reconfiguring default gamepad settings on connection
fn configure_new_gamepads(
    mut commands: Commands,
    mut connection_events: MessageReader<GamepadConnectionEvent>,
    mut gamepad_settings: Query<&mut GamepadSettings>,
) {
    // Disable broken deadzones
    // See bevy Issue #24009
    for event in connection_events.read() {
        if event.connected() {
            if let Ok(mut settings) = gamepad_settings.get_mut(event.gamepad) {
                settings.default_axis_settings.set_deadzone_lowerbound(0.0);
                settings.default_axis_settings.set_deadzone_upperbound(0.0);
            } else {
                let mut settings = GamepadSettings::default();

                settings.default_axis_settings.set_deadzone_lowerbound(0.0);
                settings.default_axis_settings.set_deadzone_upperbound(0.0);

                commands.entity(event.gamepad).insert(settings);
            }
        }
    }
}

/// System for attaching newly connected gamepads to players
fn attach_gamepads_to_players(
    mut commands: Commands,
    mut connection_events: MessageReader<GamepadConnectionEvent>,
    input_dependants: Query<&InputDependants>,
    unattached_players: Query<Entity, (With<Player>, Without<InputSource>)>
) {
    for event in connection_events.read() {
        let GamepadConnectionEvent { gamepad, connection } = event;
        match connection {
            GamepadConnection::Connected { name, vendor_id: _, product_id: _ } => {
                debug!("Gamepad \"{}\" connected as entity {}", name, gamepad);

                // If no players are reading from this gamepad...
                if let Ok(dependants) = input_dependants.get(*gamepad) && !dependants.is_empty() {
                    continue;
                }

                // ...and there is a player without a source...
                if let Some(player) = unattached_players.iter().next() {
                    // ...give that player this gamepad as a source
                    commands.entity(player).insert(InputSource(*gamepad));
                    debug!("Associated gamepad entity {} with player entity {}", gamepad, player);
                }
            },
            GamepadConnection::Disconnected => {
                debug!("Gamepad disconnected from entity {}", gamepad);
            },
        }
    }
}

/// System to attach `InputMap`s to `Gamepad`s
fn attach_input_maps(
    mut commands: Commands,
    gamepads: Query<Entity, (With<Gamepad>, Without<InputMap<GameAction>>)>
) {
    for gamepad in gamepads {
        commands
            .entity(gamepad)
            .insert(GameAction::default_gamepad_input_map()
                .with_gamepad(gamepad));
        debug!("Bound default input map to gamepad entity {}", gamepad);
    }
}

/// System for attaching newly spawned players to devices (keyboard or gamepads)
fn attach_players_to_devices(
    mut commands: Commands,
    devices: Query<(Entity, Option<&InputDependants>), With<InputMap<GameAction>>>,
    unattached_players: Query<Entity, (With<Player>, Without<InputSource>)>
) {
    // For all the players without an input device...
    for player in unattached_players {
        // ...try to find an input device with no dependants...
        if let Some(device) = devices
            .iter()
            .filter_map(|(gamepad, dependants)| {
                dependants
                    .is_none_or(|v| v.is_empty())
                    .then_some(gamepad)
            }).next() {
                // ...and attach it to that player
                commands.entity(player).insert(InputSource(device));
                debug!("Associated player entity {} with input device entity {}", player, device);
            }
    }
}
