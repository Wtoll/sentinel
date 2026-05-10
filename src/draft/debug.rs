//! Debugging tools for the application

use bevy::{color, ecs::{error::CommandWithEntity, system::entity_command}, prelude::*};

use crate::core::{AppState, GameState};

/// Plugin for enabling the game's debugging tools
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(state_hint::plugin)
            .add_systems(Update, draw_gizmos
                .run_if(in_state(GameState::Running)))
            .add_systems(OnEnter(GameState::Entering), transition_game_entering)
            .add_systems(Update, main_menu_debug_keybinds
                .run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, pause_menu_debug_keybinds
                .run_if(in_state(GameState::Paused)))
            .add_systems(Update, debug_system)
            .add_systems(Startup, initialize)
            .add_systems(Update, main_loop);
    }
}

fn initialize(
    mut commands: Commands
) {

    // let one = commands.spawn(Name::new("Test target 1")).id();
    

    // let test_holder = commands.spawn(ConnectedTo { nodes: vec!(one) }).id();



}

fn main_loop(
    mut commands: Commands
) {
    

    // if let Ok((e, con)) = query.single() {

    //     info!("Nodes Pre-Insert: {:?}", con.nodes);

    //     if con.nodes.len() < 2 {
    //         let two = commands.spawn(Name::new("Test target 2")).id();

    //         commands.entity(e).insert(ConnectedTo { nodes: vec!(two) }).id();
    //     }

    //     info!("Nodes Post-Insert: {:?}", con.nodes);
    // }

    
}




/// System that automatically transitions out of the game entering state
fn transition_game_entering(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Running);
}

/// System that handles debug keybinds on the main menu
fn main_menu_debug_keybinds(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        next_state.set(AppState::InGame);
    }

    if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(AppState::Exit);
    }
}

/// System that handles debug keybinds in the pause menu
fn pause_menu_debug_keybinds(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::Leaving);
    }
}

/// System for drawing debug gizmos in the world
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::X, color::palettes::basic::RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y, color::palettes::basic::GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z, color::palettes::basic::BLUE);
}

/// System for debugging things as needed
fn debug_system(
    mut commands: Commands,
    entities: Query<Entity>
) {
    //info!("About to log all of the components for all entities\n");

    for entity in entities {
        // log_components will error if it's run for a stale entity id, so silence any errors that happen
        //commands.queue_silenced(entity_command::log_components().with_entity(entity));
    }
}

mod state_hint {

    use bevy::prelude::*;

    use crate::core::{AppState, GameState};

    pub fn plugin(app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), initialize)
            .add_systems(Update, update.run_if(in_state(AppState::InGame)));
    }

    #[derive(Component)]
    struct StateHint;

    fn initialize(
        mut commands: Commands
    ) {
        commands.spawn((
            DespawnOnExit(AppState::InGame),
            StateHint,
            Text::new(""),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: px(10),
                left: px(10),
                ..default()
            }
        ));
    }

    fn update(
        game_state: Res<State<GameState>>,
        mut state_text: Single<&mut Text, With<StateHint>>
    ) {
        state_text.0 = format!("{:?}", game_state.get());
    }
}