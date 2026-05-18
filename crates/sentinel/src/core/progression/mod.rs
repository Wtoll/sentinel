//! Progression system
//! 

use bevy::prelude::*;

use crate::core::state::GameState;

mod graph;

/// Plugin for progression
pub fn plugin(app: &mut App) {
    app
        .init_resource::<ProgressionManager>()
        .add_systems(OnEnter(GameState::Entering), load_progression)
        .add_plugins(graph::plugin);
}

/// The global progression manager
#[derive(Resource, Default)]
pub struct ProgressionManager {
    graph: Option<Entity>
}

impl ProgressionManager {
    /// Spawns a new default progression graph if there isn't already one
    pub fn or_default(&mut self, mut commands: Commands) {
        if self.graph.is_none() {
            self.graph = Some(graph::spawn_default_graph(&mut commands).id());
        }
    }
}

fn load_progression(
    commands: Commands,
    mut progression_manager: ResMut<ProgressionManager>
) {
    progression_manager.or_default(commands);
}