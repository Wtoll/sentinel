//! The game's built-in level editor
//! 

use bevy::prelude::*;

/// Plugin for enabling the game's built-in level editor
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<EditorState>()
            .add_systems(Update, editor_keybind);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum EditorState {
    #[default]
    Game,
    Editor
}

fn editor_keybind(
    commands: Commands,
    mut next_state: ResMut<NextState<EditorState>>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) &&
        input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) &&
        input.just_pressed(KeyCode::KeyE) {
            info!("Enter editor");

            next_state.set(EditorState::Editor);
        }
}