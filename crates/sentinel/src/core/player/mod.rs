//! Global player manager
//! 

use bevy::prelude::*;

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
    fn spawn_player<'a>(&mut self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let index = self.players.len();

        let id = commands.spawn((
            Name::new(format!("Player {}", index + 1)),
            Player { index }
        )).id();

        self.players.push(id);

        commands.entity(id)
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