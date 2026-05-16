//! Adds names to essential entity archetypes
//! 

use bevy::{picking::pointer::PointerId, prelude::*, window::{Monitor, PrimaryWindow}};

pub fn plugin(app: &mut App) {
    app
        .add_systems(Last, (
            label_monitors,
            label_pointers,
            label_primary_window,
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

fn label_primary_window(
    mut commands: Commands,
    primary_windows: Query<Entity, (Added<PrimaryWindow>, Without<Name>)>
) {
    for window in primary_windows {
        commands.entity(window).insert(Name::new("Primary Window"));
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
    monitors: Query<Entity, (Added<Monitor>, Without<Name>)>,
    mut counter: Local<usize>
) {
    for monitor in monitors {
        *counter += 1;

        commands.entity(monitor).insert(Name::new(format!("Monitor {}", *counter)));
    }
}