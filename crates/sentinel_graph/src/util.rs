//! General utilities for interacting with [`Graph`]s, [`Node`]s, and
//! [`Edge`]s.
//! 

use std::iter;

use bevy::{ecs::relationship::Relationship, prelude::*};

use super::prelude::*;

/// Returns the [`Relationship`] information for an [`Entity`] if it exists.
fn relationship<R: Relationship>(world: &World, e: Entity) -> Option<Entity> {
    world
        .get::<R>(e)
        .map(|relationship| relationship.get())
}

/// Returns the [`RelationshipTarget`] information for an [`Entity`] if it exists.
fn relationship_target<R: RelationshipTarget>(world: &World, e: Entity) -> Option<&R::Collection> {
    world
        .get::<R>(e)
        .map(|relationship_target| relationship_target.collection())
}

/// Returns the [`Graph`] information for an [`Entity`] if it exists.
pub fn graph(world: &World, e: Entity) -> Option<&Vec<Entity>> {
    relationship_target::<Graph>(world, e)
}

/// Returns the [`EdgeInput`] information for an [`Entity`] if it exists.
pub fn edge_input(world: &World, e: Entity) -> Option<Entity> {
    relationship::<EdgeInput>(world, e)
}

/// Returns the [`EdgeOutput`] information for an [`Entity`] if it exists.
pub fn edge_output(world: &World, e: Entity) -> Option<Entity> {
    relationship::<EdgeOutput>(world, e)
}

/// Returns the [`EdgeGraph`] information for an [`Entity`] if it exists.
pub fn edge_graph(world: &World, e: Entity) -> Option<Entity> {
    relationship::<EdgeGraph>(world, e)
}

/// Returns the [`NodeInputs`] information for an [`Entity`] if it exists.
pub fn node_inputs(world: &World, e: Entity) -> Option<&Vec<Entity>> {
    relationship_target::<NodeInputs>(world, e)
}

/// Returns the [`NodeInputs`] information for an [`Entity`] if it exists.
pub fn node_outputs(world: &World, e: Entity) -> Option<&Vec<Entity>> {
    relationship_target::<NodeOutputs>(world, e)
}

/// Infers the parent [`Graph`] of an [`Edge`] from adjacent [`Edge`]s if possible.
pub fn infer_graph(world: &World, e: Entity) -> Option<Entity> {
    [
        edge_input(world, e),
        edge_output(world, e)
    ].into_iter()
        .flatten()
        .next()
        .and_then(|e| parent_graph(world, e))
}

/// Attempts to return the parent [`Graph`] of an [`Entity`].
/// 
/// Works for both [`Node`]s and [`Edge`]s.
pub fn parent_graph(world: &World, e: Entity) -> Option<Entity> {
    [
        node_inputs(world, e),
        node_outputs(world, e)
    ].into_iter()
        .flatten()
        .flatten().copied()
        .chain(iter::once(e))
        .flat_map(|e| edge_graph(world, e))
        .next()
}

/// Returns whether the given [`Entity`] is a part of the given [`Graph`].
pub fn in_graph(world: &World, graph_entity: Entity, entity: Entity) -> bool {
    parent_graph(world, entity).is_some_and(|entity_graph| entity_graph.eq(&graph_entity))
}