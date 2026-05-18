//! Commands for operating over graphs
//! 

// TODO: Some kind of flatten implementation that flattens a node that is a graph


use bevy::{ecs::{change_detection::MaybeLocation, entity::EntityCloner}, prelude::*};

use crate::Edge;

/// Transforms an edge into a node with two new edges in its place
/// 
/// # Behavior
/// 
/// The components of the edge are cloned to the two new edges that replace it.
pub fn promote_edge(commands: &mut Commands, edge: Entity) -> (Entity, Entity) {

    let input = commands.spawn_empty().id();
    let output = commands.spawn_empty().id();

    commands.queue(move |world: &mut World| {
        let edge_data = world.entity(edge).get::<Edge>();
        
        if edge_data.is_none() {
            return;
        }

        let edge_data = edge_data.unwrap().clone();

        world.entity_mut(edge).remove::<Edge>();

        EntityCloner::build_opt_out(world).clone_entity(edge, input);
        EntityCloner::build_opt_out(world).clone_entity(edge, output);

        world.entity_mut(input).insert(Edge::new(edge_data.input(), edge, edge_data.graph()));
        world.entity_mut(output).insert(Edge::new(edge, edge_data.output(), edge_data.graph()));
    });

    (input, output)
}

/// Creates a node that splits both edges of a cycle
/// 
/// # Behavior
/// 
/// If the entities are not edges, or the edges are not a valid cycle, the
/// existing edges will not be modified and the returned entities will be
/// despawned.
/// 
/// The components of each edge are cloned to the two new edges that
/// replace them. Then the components of the first edge are moved onto the
/// second edge to combine them into a single node. Finally, the first edge is
/// despawned.
#[track_caller]
pub fn twist_cycle(commands: &mut Commands, cycle: (Entity, Entity)) -> ((Entity, Entity), (Entity, Entity)) {
    let caller = MaybeLocation::caller();

    let zero_inwards = commands.spawn_empty().id();
    let zero_outwards = commands.spawn_empty().id();
    let one_inwards = commands.spawn_empty().id();
    let one_outwards = commands.spawn_empty().id();

    commands.queue(move |world: &mut World| {

        let zero_data = world.entity(cycle.0).get::<Edge>();
        let one_data = world.entity(cycle.1).get::<Edge>();

        let error = 'err: {
            for (data, id) in [(zero_data, cycle.0), (one_data, cycle.1)] {
                if data.is_none() {
                    break 'err Some(format!("The entity {:?} was not an edge", id));
                }
            }

            if !Edge::is_cycle((zero_data.unwrap(), one_data.unwrap())) {
                break 'err Some(format!("The edges {:?} and {:?} do not form a cycle", cycle.0, cycle.1));
            }

            None
        };

        if let Some(error) = error {
            world.entity_mut(zero_inwards).despawn();
            world.entity_mut(zero_outwards).despawn();
            world.entity_mut(one_inwards).despawn();
            world.entity_mut(one_outwards).despawn();
            warn!("{}{}. The world was not modified, and the spawned entities have been despawned.",
                caller.map(|location| format!("{location}: ")).unwrap_or_default(),
                error);
            return;
        }

        let zero_data = zero_data.unwrap().clone();
        let one_data = one_data.unwrap().clone();

        world.entity_mut(cycle.0).remove::<Edge>();
        world.entity_mut(cycle.1).remove::<Edge>();

        let mut cloner = EntityCloner::build_opt_out(world);
        cloner.clone_entity(cycle.0, zero_inwards);
        cloner.clone_entity(cycle.0, zero_outwards);
        cloner.clone_entity(cycle.1, one_inwards);
        cloner.clone_entity(cycle.1, one_outwards);

        cloner.move_components(true);
        cloner.clone_entity(cycle.0, cycle.1);
        world.entity_mut(cycle.0).despawn();

        world.entity_mut(zero_inwards).insert(Edge::new(zero_data.input(), cycle.1, zero_data.graph()));
        world.entity_mut(zero_outwards).insert(Edge::new(cycle.1, zero_data.output(), zero_data.graph()));
        world.entity_mut(one_inwards).insert(Edge::new(one_data.input(), cycle.1, one_data.graph()));
        world.entity_mut(one_outwards).insert(Edge::new(cycle.1, one_data.output(), one_data.graph()));
    });

    ((zero_inwards, zero_outwards), (one_inwards, one_outwards))
}

