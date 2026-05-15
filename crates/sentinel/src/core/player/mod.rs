//! Global player manager
//! 

use bevy::prelude::*;

use crate::core::state::AppState;

/// Plugin that enables the game's global player manager
pub struct PlayerManagerPlugin;

impl Plugin for PlayerManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerManager>();
    }
}

/// The global player manager resource
#[derive(Resource, Default)]
pub struct PlayerManager {
    players: Vec<Entity>
}

impl PlayerManager {
    /// Spawns a player in the world using the given commands
    pub fn spawn_player<'a>(
        &mut self,
        commands: &'a mut Commands
    ) -> EntityCommands<'a> {
        let index = self.players.len();

        let id = commands.spawn((
            Name::new(format!("Player {}", index + 1)),
            Player { index },
            DespawnOnExit(AppState::Game)
        )).id();

        self.players.push(id);

        let mut entity = commands.entity(id);
        entity.queue(Self::configure_player);
        entity
    }

    fn configure_player(mut entity: EntityWorldMut) {
        let mesh = entity.get_resource_mut::<Assets<Mesh>>().unwrap().add(Cuboid::new(1.0, 1.0, 1.0));
        let material = entity.get_resource_mut::<Assets<StandardMaterial>>().unwrap().add(Color::hsl(1.0, 1.0, 0.5));

        entity.insert((
            Mesh3d(mesh),
            MeshMaterial3d(material)
        ));
    }
}

/// A component marking a player
#[derive(Component)]
pub struct Player {
    /// The player's index
    /// 
    /// Zero-based, so "Player 1" will have index 0.
    index: usize,
}