use std::marker::PhantomData;

use bevy::prelude::*;

use crate::core::physics::MovementController;

#[derive(Component)]
#[require(Transform, MovementController)]
pub struct Player {
    phantom: PhantomData<()>
}

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