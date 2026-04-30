//! Level manager

use bevy::prelude::*;

use crate::core::{AppState, player::spawn_player};

/// Plugin for enabling the level manager
pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_level);
    }
}

fn spawn_level(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {



    spawn_player(
        &mut commands,
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        materials.add(Color::hsl(1.0, 1.0, 0.5))
    );

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -1.0, 0.0)
    ));
}