//! Progression system
//! 
//! 

use bevy::{ecs::error::CommandWithEntity, prelude::*};
use bevy_rand::prelude::*;
use rand::RngExt;

use crate::core::{GameState, graph::prelude::*};

/// Plugin for managing game progression
pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Entering), load_progression_graph)
            .add_systems(Update, update);
    }
}





fn load_progression_graph(
    mut commands: Commands
) {
    
    let prog_start = commands.spawn(ProgressionStart).id();
    let (prog_end, forward, backward) = commands.spawn_cyclic_with(prog_start);

    commands.entity(prog_end).insert(ProgressionEnd);

    commands.queue(move |world: &mut World| {
        let graph = parent_graph(world, forward).unwrap();

        world.entity_mut(graph).insert((GameProgression, Uninitialized));
    });
}


fn update(
    mut commands: Commands,
    start_query: Query<(Entity, &ProgressionStart)>
) {

    if let Ok((prog_start, _)) = start_query.single() {

        // commands.queue(GraphNode::debug().with_entity(prog_start));


    }



}





#[derive(Component)]
struct GameProgression;

#[derive(Component)]
struct ProgressionStart;

#[derive(Component)]
struct ProgressionEnd;

#[derive(Component)]
struct Uninitialized;
