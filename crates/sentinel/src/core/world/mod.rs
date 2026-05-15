//! Global world management
//! 

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::core::{state::{AppState, GameEnteringTask, GameState}, util::task::LifecycleTaskHolder};

/// Plugin that enables the global world manager
pub struct WorldManagerPlugin;

impl Plugin for WorldManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), load_world)
            .add_systems(OnEnter(GameState::Leaving), exit_world);
    }
}




fn load_world(
    mut commands: Commands
) {
    commands.spawn((
        LifecycleTaskHolder::new(
            AsyncComputeTaskPool::get().spawn(async move {
                String::from("Task completed")
            }),
            move |mut entity: EntityCommands, result: String| {
                info!("{}", result);
                entity.despawn();
            }
        ),
        GameEnteringTask
    ));
}


fn exit_world(
    mut commands: Commands
) {

}