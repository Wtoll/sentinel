//! An interface for producers of input data
//! 

use bevy::prelude::*;

pub mod raw;
pub mod hal;










/// A relationship component containing a set of other entities that depend on
/// this entity as a source of user input.
#[derive(Component)]
#[relationship_target(relationship = super::sink::InputSource)]
pub struct InputSinks(Vec<Entity>);


