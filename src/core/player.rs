//! Player entities

use std::marker::PhantomData;

use bevy::prelude::*;

use crate::core::physics::MovementController;

/// A player entity
#[derive(Component)]
#[require(Transform, MovementController)]
pub struct Player {
    phantom: PhantomData<()>
}

/// Helper function for spawning a player in the world with the correct
/// component set
pub fn spawn_player<M: Material>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<M>
) {
    commands.spawn((
        Player {
            phantom: PhantomData::default()
        },
        Mesh3d(mesh),
        MeshMaterial3d(material)
    ));
}