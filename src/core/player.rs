//! Player entities

use bevy::prelude::*;

use crate::core::physics::MovementController;

/// A player entity
#[derive(Component)]
#[require(Transform, MovementController)]
pub struct Player {
    index: usize
}

pub use manager::PlayerManager;

mod manager {
    use bevy::prelude::*;

    use crate::core::{AppState, Player};

    /// A resource for managing the state of players
    #[derive(Resource, Default)]
    pub struct PlayerManager {
        count: usize
    }

    impl PlayerManager {
        fn next_index(&mut self) -> usize {
            self.count += 1;
            self.count
        }

        /// Spawns a new player into the world with the given bundle
        pub fn spawn_player_with<B: Bundle>(bundle: B) -> impl Command {
            move |world: &mut World| {
                let index = world.get_resource_or_init::<PlayerManager>().next_index();

                let mesh = world.get_resource_mut::<Assets<Mesh>>().unwrap().add(Cuboid::new(1.0, 1.0, 1.0));
                let material = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap().add(Color::hsl(1.0, 1.0, 0.5));

                world.spawn((
                    Name::new(format!("Player {}", index)),
                    Player {
                        index
                    },
                    DespawnOnExit(AppState::InGame),
                    Mesh3d(mesh),
                    MeshMaterial3d(material)
                )).insert(bundle);
            }
        }

        /// Spawns a new player into the world
        pub fn spawn_player() -> impl Command {
            Self::spawn_player_with(())
        }
    }

}