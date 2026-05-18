//! Adds names to essential entity archetypes
//! 

use bevy::{picking::pointer::PointerId, prelude::*, window::{Monitor, PrimaryWindow}};

pub fn plugin(app: &mut App) {
    app
        .add_systems(Last, (
            label_monitors,
            label_pointers,
            label_observers
        ));
}

fn label_observers(
    mut commands: Commands,
    observers: Query<Entity, (Added<Observer>, Without<Name>)>
) {
    for observer in observers {
        commands.entity(observer).insert(Name::new("Observer"));
    }
}

fn label_pointers(
    mut commands: Commands,
    pointers: Query<Entity, (Added<PointerId>, Without<Name>)>
) {
    for pointer in pointers {
        commands.entity(pointer).insert(Name::new("Pointer"));
    }
}

fn label_monitors(
    mut commands: Commands,
    monitors: Query<(Entity, &Monitor), (Added<Monitor>, Without<Name>)>,
    mut counter: Local<usize>
) {
    for (id, monitor) in monitors {
        *counter += 1;

        commands.entity(id).insert(Name::new(if let Some(name) = &monitor.name {
            format!("Monitor {}", name.clone())
        } else {
            format!("Monitor {}", *counter)
        }));
    }
}