//! Plugin for enabling the game's built-in level editor
//! 

use bevy::prelude::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>();
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum EditorState {
    #[default]
    Game,
    Editor
}