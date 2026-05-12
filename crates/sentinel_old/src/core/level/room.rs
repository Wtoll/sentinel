//! Rooms in levels
//! 

use bevy::{camera::primitives::Aabb, prelude::*};

/// A room in a level of the game
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Room {
    /// The volume of a room
    pub volume: RoomVolume
}

/// The volume a room and its children take up
#[derive(Default, Reflect)]
pub struct RoomVolume {
    aabb: Aabb
}

impl RoomVolume {
    /// Creates a simple volume for a room
    pub fn simple(size: Vec3) -> Self {
        Self::from(
            Aabb::from_min_max(Vec3::ZERO, size)
        )
    }
}

impl From<Aabb> for RoomVolume {
    fn from(aabb: Aabb) -> Self {
        Self { aabb }
    }
}

impl AsRef<Aabb> for RoomVolume {
    fn as_ref(&self) -> &Aabb {
        &self.aabb
    }
}