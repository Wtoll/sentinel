//! An interface for consumers of user input
//! 

use bevy::prelude::*;

pub mod combo;
pub mod profile;

/// A relationship component that provides a source of user input from another
/// entity.
#[derive(Component)]
#[relationship(relationship_target = super::src::InputSinks)]
pub struct InputSource(pub Entity);