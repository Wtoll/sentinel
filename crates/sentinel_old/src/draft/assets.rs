//! Game assets

use bevy::prelude::*;

/// Plugin for enabling the management of game assets
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_assets);
    }
}

fn initialize_assets(
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {

}