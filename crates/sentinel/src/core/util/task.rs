//! Utilities for working with tasks
//! 
//! TODO: Re-evaluate this
//! 

use bevy::{ecs::component::{Immutable, StorageType}, prelude::*, tasks::{Task, futures::now_or_never}};

/// Plugin that enables task management utilities
pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, poll_tasks);
    }
}

fn poll_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut LifecycleTaskHolder)>
) {
    for (entity, mut task) in &mut tasks {
        task.poll(commands.entity(entity));
    }
}

/// A component that polls a [`Task`] until it completes, passes the return
/// value and commands to a callback, and then removes itself from the entity.
#[derive(Component)]
pub struct LifecycleTaskHolder(Box<dyn ErasedLifecycleTask>);

impl LifecycleTaskHolder {
    /// Creates a new [`LifecycleTaskHolder`] from the given task and callback
    pub fn new<T: Send + 'static>(task: Task<T>, callback: impl Fn(EntityCommands, T) + 'static + Send + Sync) -> Self {
        Self(Box::new(LifecycleTask::new(task, callback)))
    }

    fn poll(&mut self, mut entity: EntityCommands) {
        if self.0.is_finished() {
            entity.remove::<LifecycleTaskHolder>();
            self.0.complete(entity);
        }
    }
}

trait ErasedLifecycleTask: Send + Sync {
    fn is_finished(&self) -> bool;

    fn complete(&mut self, entity: EntityCommands);
}

struct LifecycleTask<T: Send> {
    task: Task<T>,
    callback: Box<dyn Fn(EntityCommands, T) + Send + Sync>
}

impl<T: Send> LifecycleTask<T> {
    fn new(task: Task<T>, callback: impl Fn(EntityCommands, T) + 'static + Send + Sync) -> Self {
        Self { task, callback: Box::new(callback) }
    }
}

impl<T: Send> ErasedLifecycleTask for LifecycleTask<T> {
    fn is_finished(&self) -> bool {
        self.task.is_finished()
    }
    
    fn complete(&mut self, entity: EntityCommands) {
        if let Some(result) = now_or_never(&mut self.task) {
            (self.callback)(entity, result);
        }
    }
}



