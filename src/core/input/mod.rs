use bevy::{input::gamepad::{GamepadConnection, GamepadConnectionEvent}, prelude::*};

use crate::core::{MenuState, Player, app_state::scheduling::GameSystemSet, input::scheduling::InputSystemSet};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                configure_new_gamepads,
                assign_new_gamepads,
                process_gamepads)
                    .in_set(GameSystemSet)
                    .in_set(InputSystemSet))
            .add_systems(Update, 
                handle_keyboard_input
                    .in_set(InputSystemSet))
            .configure_sets(Update, scheduling::AfterInputSystemSet
                .after(InputSystemSet));
    }
}

pub mod scheduling {
    use bevy::prelude::*;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct InputSystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AfterInputSystemSet;
}
















































/// Component for storing a source of user input data
#[derive(Component)]
pub struct InputSource(pub Entity);

/// Component for storing user inputs after mapping
#[derive(Component, Default, Clone)]
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

impl GameInput {
    fn write_to(&self, target: &mut Mut<'_, GameInput>) {
        **target = self.clone();
    }
}

/// System for handling keyboard input
fn handle_keyboard_input(
    menu_state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<MenuState>>,
    key_codes: Res<ButtonInput<KeyCode>>
) {
    if (key_codes.just_pressed(KeyCode::KeyE)) {
        if let MenuState::MainMenu = menu_state.get() {
            next_state.set(MenuState::InGame);
        }
    }
}

/// System for configuring gamepads that reconnect
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

/// System for assigning new gamepads to players
fn assign_new_gamepads(
    mut commands: Commands,
    mut connection_events: MessageReader<GamepadConnectionEvent>,
    players: Query<(Entity, Option<&InputSource>), With<Player>>
) {
    for event in connection_events.read() {
        let GamepadConnectionEvent { gamepad, connection } = event;
        match connection {
            GamepadConnection::Connected { name: _, vendor_id: _, product_id: _ } => {
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

/// System for processing gamepad input and remapping it
fn process_gamepads(
    mut commands: Commands,
    gamepads: Query<(Entity, &Gamepad, Option<&mut GameInput>)>
) {
    for (entity, gamepad, game_input) in gamepads {
        let new_game_input = GameInput::from(gamepad);

        if let Some(mut game_input) = game_input {
            new_game_input.write_to(&mut game_input);
        } else {
            commands.entity(entity).insert(new_game_input);
        }
    }
}