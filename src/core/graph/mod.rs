//! Graph data structures in Bevy ECS
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 



/// Helpful imports for interacting with [`Graph`]s, [`Node`]s, and [`Edge`]s.
pub mod prelude {
    pub use super::{
        Graph,
        Edge,
        EdgeGraph,
        EdgeInput,
        EdgeOutput,
        Node,
        NodeInputs,
        NodeOutputs
    };

    pub use super::util::{
        parent_graph,
        edge_graph,
        edge_input,
        edge_output,
        node_inputs,
        node_outputs,
        in_graph
    };

    pub use super::commands::CommandsExt;
}

mod core {
    use bevy::{ecs::error::CommandWithEntity, prelude::*};

    use crate::core::graph::util::graph;

use super::util::infer_graph;

    use super::prelude::*;

    /// A [`Graph`] comprised of [`Edge`]s between [`Node`]s
    #[derive(Component, Default)]
    #[relationship_target(relationship = EdgeGraph, linked_spawn)]
    pub struct Graph(Vec<Entity>);

    impl Graph {
        /// Flattens a [`Node`] that is itself a [`Graph`] using the given
        /// [`Node`]s in the subgraph as proxies for its parents replacement.
        /// 
        /// Note: `input_node` is allowed to equal `output_node`.
        pub fn try_flatten_with(input_node: Entity, output_node: Entity) -> impl EntityCommand {
            move |mut entity: EntityWorldMut| {
                let graph_node = entity.id();
                let world = entity.into_world_mut();

                if in_graph(world, graph_node, input_node) && in_graph(world, graph_node, output_node) {
                    if let Some(new_graph) = parent_graph(world, graph_node) {
                        // Move all the edges in the subgraph to the new parent graph
                        if let Some(edges) = graph(world, graph_node) {
                            edges.clone().into_iter().for_each(|edge| {
                                world.entity_mut(edge).insert(EdgeGraph(new_graph));
                            });
                        }
                        // Redirect all inputs to the target node to the new input node instead
                        if let Some(inputs) = node_inputs(world, graph_node) {
                            inputs.clone().into_iter().for_each(|edge| {
                                world.entity_mut(edge).insert(EdgeOutput(input_node));
                            });
                        }
                        // Redirect all outputs from the target node to the new output node instead
                        if let Some(outputs) = node_outputs(world, graph_node) {
                            outputs.clone().into_iter().for_each(|edge| {
                                world.entity_mut(edge).insert(EdgeInput(output_node));
                            });
                        }
                    }
                }
            }
        }
    }

    /// An [`Edge`] within a [`Graph`].
    #[derive(Component)]
    #[relationship(relationship_target = Graph)]
    pub struct EdgeGraph(Entity);

    impl EdgeGraph {
        /// Infers which [`Graph`] an [`Edge`] belongs to then assigns its
        /// [`EdgeGraph`] component accordingly.
        /// 
        /// When run for a properly constructed [`Edge`] component, this
        /// command does nothing.
        pub fn infer_graph_assign() -> impl EntityCommand {
            move |mut entity: EntityWorldMut| {
                let graph = infer_graph(entity.world(), entity.id()).unwrap_or_else(|| {
                    // SAFETY: The statement doesn't mutate anything about the
                    // entity stored within the context.
                    unsafe { entity.world_mut().spawn_empty().id() }
                });

                entity.insert(EdgeGraph(graph));
            }
        }
    }

    /// An [`Edge`] of a [`Graph`]
    #[derive(Bundle)]
    pub struct Edge {
        input: EdgeInput,
        output: EdgeOutput
    }

    impl Edge {
        /// Uses the provided commands to spawn a new [`Edge`].
        pub fn spawn_edge<'a>(commands: &'a mut Commands, from: Entity, to: Entity) -> EntityCommands<'a> {
            let edge = commands.spawn(
                Edge {
                    input: EdgeInput(from),
                    output: EdgeOutput(to)
                }
            ).id();

            commands.queue(EdgeGraph::infer_graph_assign().with_entity(edge));

            commands.entity(edge)
        }
    }

    /// The input [`Node`] of an [`Edge`].
    #[derive(Component)]
    #[relationship(relationship_target = NodeOutputs)]
    pub struct EdgeInput(Entity);

    /// The output [`Node`] of an [`Edge`].
    #[derive(Component)]
    #[relationship(relationship_target = NodeInputs)]
    pub struct EdgeOutput(Entity);

    /// A [`Node`] within a [`Graph`].
    #[derive(Component)]
    #[require(NodeInputs, NodeOutputs)]
    pub struct Node;

    /// The input [`Edge`]s of a [`Node`].
    #[derive(Component, Default)]
    #[relationship_target(relationship = EdgeOutput)]
    pub struct NodeInputs(Vec<Entity>);

    /// The output [`Edge`]s of a [`Node`].
    #[derive(Component, Default)]
    #[relationship_target(relationship = EdgeInput)]
    pub struct NodeOutputs(Vec<Entity>);
}

pub use core::{
    Graph,
    Edge,
    EdgeGraph,
    EdgeInput,
    EdgeOutput,
    Node,
    NodeInputs,
    NodeOutputs
};

/// General utilities for interacting with [`Graph`]s, [`Node`]s, and
/// [`Edge`]s.
pub mod util {
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
            .flatten()
            .map(|e| *e)
            .chain(iter::once(e))
            .flat_map(|e| edge_graph(world, e))
            .next()
    }

    /// Returns whether the given [`Entity`] is a part of the given [`Graph`].
    pub fn in_graph(world: &World, graph_entity: Entity, entity: Entity) -> bool {
        parent_graph(world, entity).is_some_and(|entity_graph| entity_graph.eq(&graph_entity))
    }
}

/// [`Command`]s and extensions to [`Commands`] for interacting with
/// [`Graph`]s, [`Node`]s, and [`Edge`]s.
pub mod commands {
    use bevy::prelude::*;

    use super::prelude::*;

    /// Helper methods for spawning [`Node`]s using [`Commands`].
    pub trait CommandsExt {
        /// Spawns a new [`Edge`] between two entities.
        fn spawn_edge(&mut self, from: Entity, to: Entity) -> EntityCommands<'_>;

        /// Spawns a new [`Node`] with a connection from the given [`Node`].
        /// 
        /// Returns a tuple containing the new [`Node`] and the new [`Edge`]
        /// between them.
        /// 
        /// See [`spawn_connected_to`](Self::spawn_connected_to) and
        /// [`spawn_cyclic_with`](Self::spawn_cyclic_with).
        /// 
        /// ## Example
        /// 
        /// ```
        /// # use bevy::prelude::*;
        /// # use sentinel::core::graph::prelude::*;
        /// #
        /// # let mut world = World::new();
        /// # let mut commands = world.commands();
        /// #
        /// let node_a = commands.spawn(Name::new("Node A")).id();
        ///
        /// let (node_b, edge_a_b) = commands.spawn_connected_from(node_a);
        ///
        /// assert_eq!(
        ///     edge_input(&world, edge_a_b)
        ///         .and_then(|input| world.get::<Name>(input))
        ///         .unwrap()
        ///         .as_str(),
        ///     "Node A"
        /// );
        /// ```
        /// 
        /// See [`edge_input`](super::util::edge_input)
        /// 
        fn spawn_connected_from(&mut self, from: Entity) -> (Entity, Entity);

        /// Spawns a new [`Node`] with a connection to the given [`Node`].
        /// 
        /// Returns a tuple containing the new [`Node`] and the new [`Edge`]
        /// between them.
        fn spawn_connected_to(&mut self, to: Entity) -> (Entity, Entity);

        /// Spawns a new [`Node`] with cyclic connections to the given
        /// [`Node`].
        /// 
        /// Returns a tuple containing the new [`Node`], and the two [`Edge`]s
        /// in directional order starting from the given [`Node`].
        fn spawn_cyclic_with(&mut self, a: Entity) -> (Entity, Entity, Entity);
    }

    impl CommandsExt for Commands<'_, '_> {
        fn spawn_edge(&mut self, from: Entity, to: Entity) -> EntityCommands<'_> {
            Edge::spawn_edge(self, from, to)
        }

        fn spawn_connected_from(&mut self, from: Entity) -> (Entity, Entity) {
            let to = self.spawn_empty().id();
            (to, self.spawn_edge(from, to).id())
        }

        fn spawn_connected_to(&mut self, to: Entity) -> (Entity, Entity) {
            let from = self.spawn_empty().id();
            (from, self.spawn_edge(from, to).id())
        }

        fn spawn_cyclic_with(&mut self, a: Entity) -> (Entity, Entity, Entity) {
            let b = self.spawn_empty().id();
            (b, self.spawn_edge(a, b).id(), self.spawn_edge(b, a).id())
        }
    }

    


    


    
}

