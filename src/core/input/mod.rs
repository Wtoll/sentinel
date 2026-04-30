//! Input Handling
//! 

use bevy::{input::gamepad::{GamepadConnection, GamepadConnectionEvent}, prelude::*};

use leafwing_input_manager::prelude::*;

use crate::core::{AppState, Player, input::scheduling::InputSystemSet};

/// Plugin to enable the game's input processing systems
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_keyboard_input,
                configure_new_gamepads,
                attach_input_maps,
                attach_gamepads_to_players,
                attach_players_to_gamepads)
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

/// An action that may be performed by a player in the world of the game
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameAction {
    /// Lateral movement
    #[actionlike(Axis)]
    MoveLateral,
    /// Jump action
    Jump,
}

impl GameAction {
    fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_axis(Self::MoveLateral, GamepadAxis::LeftStickX)
            .with(Self::Jump, GamepadButton::South)
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

/// System for handling keyboard input
fn handle_keyboard_input(
    menu_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    key_codes: Res<ButtonInput<KeyCode>>
) {
    if key_codes.just_pressed(KeyCode::KeyE) {
        if let AppState::MainMenu = menu_state.get() {
            next_state.set(AppState::InGame);
        }
    }
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
            .insert(GameAction::default_input_map()
                .with_gamepad(gamepad));
        debug!("Bound default input map to gamepad entity {}", gamepad);
    }
}

/// System for attaching newly spawned players to gamepads
fn attach_players_to_gamepads(
    mut commands: Commands,
    gamepads: Query<(Entity, Option<&InputDependants>), With<Gamepad>>,
    unattached_players: Query<Entity, (With<Player>, Without<InputSource>)>
) {
    // For all the players without a gamepad...
    for player in unattached_players {
        // ...try to find a gamepad with no dependants...
        if let Some(gamepad) = gamepads
            .iter()
            .filter_map(|(gamepad, dependants)| {
                dependants
                    .is_none_or(|v| v.is_empty())
                    .then_some(gamepad)
            }).next() {
                // ...and attach it to that player
                commands.entity(player).insert(InputSource(gamepad));
                debug!("Associated player entity {} with gamepad entity {}", player, gamepad);
            }
    }
}
