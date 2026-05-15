use bevy::{ecs::error::CommandWithEntity, prelude::*};

use super::util::graph;

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
        move |entity: EntityWorldMut| {
            let graph_node = entity.id();
            let world = entity.into_world_mut();

            if in_graph(world, graph_node, input_node)
                && in_graph(world, graph_node, output_node)
                && let Some(new_graph) = parent_graph(world, graph_node) {
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

impl Node {
    /// Prints debug information for the node to the log
    pub fn debug() -> impl EntityCommand {
        move |entity: EntityWorldMut| {

            let parent_graph = parent_graph(entity.world(), entity.id()).unwrap();

            let node_inputs = node_inputs(entity.world(), entity.id())
                .map(|inputs| {
                    inputs.iter().filter_map(|input| {
                        edge_input(entity.world(), *input)
                    }).collect::<Vec<Entity>>()
                });

            let node_outputs = node_outputs(entity.world(), entity.id())
                .map(|outputs| {
                    outputs.iter().filter_map(|output| {
                        edge_output(entity.world(), *output)
                    }).collect::<Vec<Entity>>()
                });

            info!("Graph {}", parent_graph);
            info!("┬");

            if let Some(node_inputs) = node_inputs {
                for input in node_inputs {
                    info!("├ Node {}", input);
                }
            }

            info!("└ Node {}", entity.id());

            if let Some(node_outputs) = node_outputs {
                for i in 0..node_outputs.len() {
                    if i != node_outputs.len() - 1 {
                        info!("  ├ Node {}", node_outputs[i]);
                    } else {
                        info!("  └ Node {}", node_outputs[i]);
                    }
                }
            }
        }
    }
}

/// The input [`Edge`]s of a [`Node`].
#[derive(Component, Default)]
#[relationship_target(relationship = EdgeOutput)]
pub struct NodeInputs(Vec<Entity>);

/// The output [`Edge`]s of a [`Node`].
#[derive(Component, Default)]
#[relationship_target(relationship = EdgeInput)]
pub struct NodeOutputs(Vec<Entity>);

/// Type alias to disambiguate [`Node`] when also importing [`bevy::prelude`].
pub type GraphNode = Node;