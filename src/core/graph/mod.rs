//! Graph data structures in Bevy ECS
//! 
//! 
//! 
//! 
//! ## Unintended Behavior
//! 
//! Bevy doesn't yet have strong restrictions on which component types can
//! exist on the same entity. For that reason it is possible to create some
//! questionable structures if you're really dedicated. Thankfully, you
//! shouldn't run into any unintended behavior so long as you ensure the
//! following:
//! 1. All edge components ([`Edge`], [`EdgeInput`], and [`EdgeOutput`]) are
//! mutually exclusive with all node components ([`Node`], [`NodeInputs`],
//! [`NodeOutputs`])
//! 2. All edge components ([`Edge`], [`EdgeInput`], and [`EdgeOutput`]) are
//! mutually exclusive with all graph components ([`Graph`], [`GraphNodes`],
//! [`GraphEdges`])
//! 
//! It is okay, however, for a [`Node`] to also be a [`Graph`]. Otherwise,
//! intended behavior can simply be inferred from any strange cases depending
//! on the level of correctness desired. Take for example, an [`EdgeOutput`]
//! where the stored [`Entity`] no longer has a [`Node`]. The other components
//! associated with that [`Entity`] may still exist, in which case 
//! 
//! 
//! 
//! 



/// Helpful stuff
pub mod prelude {

    pub use super::{
        Graph,
        GraphEdges,
        GraphNodes,
        Edge,
        EdgeInput,
        EdgeOutput,
        Node,
        NodeInputs,
        NodeOutputs
    };

    pub use super::commands::{
        CommandsExt,
        EntityCommandsExt
    };

    pub use super::util::{
        common_graph,
        node_graph,
        node_inputs,
        node_outputs,
        edge_input,
        edge_output
    };
}





mod test {
    use bevy::prelude::*;

    use super::{Graph, GraphEdges, GraphNodes, Edge, EdgeInput, EdgeOutput, Node, NodeInputs, NodeOutputs, commands::CommandsExt};

    /// Plugin for testing graph behavior
    pub struct GraphTestPlugin;

    impl Plugin for GraphTestPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_systems(Startup, startup)
                .add_systems(Update, update);
        }
    }

    fn startup(
        mut commands: Commands
    ) {

        let node_a = commands
            .spawn_headless_node()
            .insert(Name::new("Node A"))
            .id();

        let (node_b, edge_a_b) = commands.spawn_node_from(node_a);

        commands.entity(node_b).insert(Name::new("Node B"));

        let (node_c, edge_b_c, edge_c_b) = commands.spawn_node_cyclic(node_b);

        commands.entity(node_c).insert(Name::new("Node C"));

        let (node_d, edge_c_d) = commands.spawn_node_from(node_c);

        commands.entity(node_d).insert(Name::new("Node D"));

        commands.entity(edge_b_c).despawn();
    }


    fn update(
        mut commands: Commands,
        graph_query: Query<(Entity, &Graph, &GraphEdges, &GraphNodes)>,
        node_query: Query<(Entity, &Node, &NodeInputs, &NodeOutputs)>,
        edge_query: Query<(Entity, &Edge, &EdgeInput, &EdgeOutput)>
    ) {

        if let Ok((graph_entity, _, graph_edges, graph_nodes)) = graph_query.single() {

            info!("Found graph {}", graph_entity);
            info!("    With edges");
            graph_edges.iter().for_each(|edge| info!("    Edge: {}", edge));
            info!("    With nodes");
            graph_nodes.iter().for_each(|node| info!("    Node: {}", node));
        }
    }
}

pub use test::GraphTestPlugin;




















/// General utilities for interacting with [`Graph`]s, [`Node`]s, and
/// [`Edge`]s.
pub mod util {
    use bevy::{ecs::entity::{EntitySet, UniqueEntityIter}, prelude::*};

    use thiserror::Error;

    use super::{EdgeInput, EdgeOutput, Node, NodeInputs, NodeOutputs};

    /// An [`Error`](std::err::Error) that occurs when an [`Entity`] does not
    /// have a [`Node`].
    #[derive(Debug, Error)]
    #[error("Entity {} is not a node", e)]
    pub struct InvalidNodeError {
        e: Entity
    }

    /// An [`Error`](std::err::Error) that occurs when two
    /// [`Node`]s are a part of different
    /// [`Graph`](super::Graph)s.
    #[derive(Debug, Error)]
    #[error("Connections cannot span across graphs, but node {} is in graph {} and node {} is in graph {}", node_a, graph_a, node_b, graph_b)]
    pub struct DifferentGraphsError {
        node_a: Entity,
        graph_a: Entity,
        node_b: Entity,
        graph_b: Entity
    }

    /// An [`Error`](std::err::Error) that occurs when an [`Edge`](super::Edge)
    /// does not posess both an [`EdgeInput`] and an [`EdgeOutput`].
    #[derive(Debug, Error)]
    pub enum PartialEdgeError {
        /// The [`Edge`](super::Edge) does not have an [`EdgeInput`].
        #[error("Edge {} is missing an input", .0)]
        MissingInput(Entity),
        /// The [`Edge`](super::Edge) does not have an [`EdgeOutput`].
        #[error("Edge {} is missing an output", .0)]
        MissingOutput(Entity)
    }

    /// An [`Error`](std::err::Error) that occurs when trying to make a
    /// connection between two [`Node`]s.
    #[derive(Debug, Error)]
    pub enum NodeConnectionError {
        /// The [`Node`]s do not have a common parent [`Graph`](super::Graph).
        /// 
        /// See [`CommonGraphError`].
        #[error(transparent)]
        NoCommonGraph(#[from] CommonGraphError),
        /// The [`Edge`](super::Edge) is missing an [`EdgeInput`] or
        /// an [`EdgeOutput`].
        /// 
        /// See [`PartialEdgeError`].
        #[error(transparent)]
        PartialEdge(#[from] PartialEdgeError),
    }

    /// An [`Error`](std::err::Error) that occurs when trying to find a common
    /// parent [`Graph`] between two [`Node`]s.
    #[derive(Debug, Error)]
    pub enum CommonGraphError {
        /// One of the [`Node`]s is invalid.
        /// 
        /// See [`InvalidNodeError`].
        #[error(transparent)]
        InvalidNode(#[from] InvalidNodeError),
        /// The [`Node`]s are on different graphs.
        /// 
        /// See [`DifferentGraphsError`].
        #[error(transparent)]
        DifferentGraphs(#[from] DifferentGraphsError),
    }

    /// Attempts to find a common parent [`Graph`](super::Graph) shared by the
    /// two [`Node`]s.
    /// 
    /// Fails if either [`Entity`] doesn't have a [`Node`], or the [`Node`]s do
    /// not share a common parent [`Graph`](super::Graph).
    /// 
    /// See [`CommonGraphError`].
    pub fn common_graph(world: &World, a: Entity, b: Entity) -> Result<Entity, CommonGraphError> {
        node_graph(world, a).map_err(From::from).and_then(|graph_a| {
            node_graph(world, b).map_err(From::from).and_then(|graph_b| {
                (graph_a == graph_b)
                    .then_some(graph_a)
                    .ok_or(DifferentGraphsError { node_a: a, graph_a, node_b: b, graph_b }.into())
            })
        })
    }

    /// Attempts to return the parent [`Graph`](super::Graph) of a [`Node`].
    /// 
    /// Fails if the [`Entity`] doesn't have a [`Node`].
    /// 
    /// See [`InvalidNodeError`].
    pub fn node_graph(world: &World, e: Entity) -> Result<Entity, InvalidNodeError> {
        world
            .get::<Node>(e)
            .ok_or(InvalidNodeError { e })
            .map(|node| node.0)
    }

    /// Attempts to return the input [`Node`] of an [`Edge`](super::Edge).
    /// 
    /// Fails if the [`Entity`] doesn't have an [`EdgeInput`].
    /// 
    /// See [`PartialEdgeError`].
    pub fn edge_input(world: &World, e: Entity) -> Result<Entity, PartialEdgeError> {
        world
            .get::<EdgeInput>(e)
            .ok_or(PartialEdgeError::MissingInput(e))
            .map(|edge_input| edge_input.0)
    }

    /// Attempts to return the output [`Node`] of an [`Edge`](super::Edge).
    /// 
    /// Fails if the [`Entity`] doesn't have an [`EdgeOutput`].
    /// 
    /// See [`PartialEdgeError`].
    pub fn edge_output(world: &World, e: Entity) -> Result<Entity, PartialEdgeError> {
        world
            .get::<EdgeOutput>(e)
            .ok_or(PartialEdgeError::MissingOutput(e))
            .map(|edge_output| edge_output.0)
    }

    fn node_inputs_unchecked(world: &World, e: Entity) -> impl EntitySet {
        // SAFETY: Bevy relationships ensure that the content of
        // relationship targets are unique sets.
        unsafe {
            UniqueEntityIter::from_iterator_unchecked(
                world.get::<NodeInputs>(e)
                    .map(|inputs| {
                        inputs.iter()
                    }).unwrap_or_default())
        }
    }

    /// Attempts to return the input [`Edge`](super::Edge)s of a [`Node`].
    /// 
    /// Fails if the [`Entity`] is not a [`Node`].
    /// 
    /// See [`InvalidNodeError`].
    pub fn node_inputs(world: &World, e: Entity) -> Result<impl EntitySet, InvalidNodeError> {
        world
            .get::<Node>(e)
            .ok_or(InvalidNodeError { e })
            .map(|_| {
                node_inputs_unchecked(world, e)
            })
    }

    fn node_outputs_unchecked(world: &World, e: Entity) -> impl EntitySet {
        // SAFETY: Bevy relationships ensure that the content of
        // relationship targets are unique sets.
        unsafe {
            UniqueEntityIter::from_iterator_unchecked(
                world.get::<NodeOutputs>(e)
                    .map(|outputs| {
                        outputs.iter()
                    }).unwrap_or_default())
        }
    }

    /// Attempts to return the output [`Edge`](super::Edge)s of a [`Node`].
    /// 
    /// Fails if the [`Entity`] is not a [`Node`].
    /// 
    /// See [`InvalidNodeError`].
    pub fn node_outputs(world: &World, e: Entity) -> Result<impl EntitySet, InvalidNodeError> {
        world
            .get::<Node>(e)
            .ok_or(InvalidNodeError { e })
            .map(|_| {
                node_outputs_unchecked(world, e)
            })
    }

}

/// [`Command`]s and extensions to [`Commands`] for interacting with
/// [`Graph`]s, [`Node`]s, and [`Edge`]s.
pub mod commands {
    use bevy::{ecs::error::CommandWithEntity, prelude::*};

    use super::{
        Graph, Edge, EdgeInput, EdgeOutput, Node,
        util::{InvalidNodeError, NodeConnectionError, common_graph, node_graph, edge_input, edge_output}
    };

    fn spawn_edge<'a>(commands: &'a mut Commands, from: Entity, to: Entity) -> EntityCommands<'a> {
        let edge = commands.spawn((EdgeInput(from), EdgeOutput(to))).id();
        commands.queue(validate_edge().with_entity(edge));
        commands.entity(edge)
    }

    /// Validates that an edge is constructed properly, and fixes an unset parent graph if possible.
    pub fn validate_edge() -> impl EntityCommand<Result<(), NodeConnectionError>> {
        move |mut world: EntityWorldMut| {
            let input: Entity = edge_input(world.world(), world.id())?;
            let output = edge_output(world.world(), world.id())?;

            common_graph(world.world(), input, output)
                .map_err(From::from)
                .map(|graph| {
                    world.insert(Edge(graph));
                })
        }
    }

    fn spawn_node_in_graph<'a>(commands: &'a mut Commands, graph: Entity) -> EntityCommands<'a> {
        commands.spawn(Node(graph))
    }

    fn spawn_headless_node<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
        let graph = commands.spawn(Graph).id();
        spawn_node_in_graph(commands, graph)
    }

    fn spawn_node_in_common<'a>(commands: &'a mut Commands, node: Entity) -> EntityCommands<'a> {
        let new = commands.spawn_empty().id();
        commands.queue(make_node_common_to(node).with_entity(new));
        commands.entity(new)
    }

    /// Makes a node share a common parent graph with another node
    pub fn make_node_common_to(reference: Entity) -> impl EntityCommand<Result<(), InvalidNodeError>> {
        move |mut world: EntityWorldMut| {
            let graph = node_graph(world.world(), reference)?;

            world
                .insert(Node(graph));
            
            Ok(())
        }
    }

    fn spawn_node_from(commands: &mut Commands, from: Entity) -> (Entity, Entity) {
        let to = spawn_node_in_common(commands, from).id();
        let edge = spawn_edge(commands, from, to).id();
        (to, edge)
    }

    fn spawn_node_to(commands: &mut Commands, to: Entity) -> (Entity, Entity) {
        let from = spawn_node_in_common(commands, to).id();
        let edge = spawn_edge(commands, from, to).id();
        (from, edge)
    }

    fn spawn_node_cyclic(commands: &mut Commands, a: Entity) -> (Entity, Entity, Entity) {
        let b = spawn_node_in_common(commands, a).id();
        let a_b = spawn_edge(commands, a, b).id();
        let b_a = spawn_edge(commands, b, a).id();
        (b, a_b, b_a)
    }

    /// Extensions to commands
    pub trait CommandsExt {
        /// Spawns an [`Edge`] going from one [`Node`] to another [`Node`].
        fn spawn_edge(&mut self, from: Entity, to: Entity) -> EntityCommands<'_>;

        /// Spawns a new [`Node`] in its own [`Graph`].
        fn spawn_headless_node(&mut self) -> EntityCommands<'_>;

        /// Spawns a new [`Node`] with a connection from the given [`Node`].
        fn spawn_node_from(&mut self, from: Entity) -> (Entity, Entity);

        /// Spawns a new [`Node`] with a connection to the given [`Node`].
        fn spawn_node_to(&mut self, to: Entity) -> (Entity, Entity);

        /// Spawns a new [`Node`] that is cyclic with the given [`Node`].
        fn spawn_node_cyclic(&mut self, a: Entity) -> (Entity, Entity, Entity);
    }

    impl CommandsExt for Commands<'_, '_> {
        fn spawn_edge(&mut self, from: Entity, to: Entity) -> EntityCommands<'_> {
            spawn_edge(self, from, to)
        }
    
        fn spawn_headless_node(&mut self) -> EntityCommands<'_> {
            spawn_headless_node(self)
        }
    
        fn spawn_node_from(&mut self, from: Entity) -> (Entity, Entity) {
            spawn_node_from(self, from)
        }
    
        fn spawn_node_to(&mut self, to: Entity) -> (Entity, Entity) {
            spawn_node_to(self, to)
        }
    
        fn spawn_node_cyclic(&mut self, a: Entity) -> (Entity, Entity, Entity) {
            spawn_node_cyclic(self, a)
        }
    }

    /// Entity command extension
    pub trait EntityCommandsExt {

    }

    impl EntityCommandsExt for EntityCommands<'_> {

    }
}

mod core {
    use bevy::prelude::*;

    /// A marker component for a [`Graph`]
    #[derive(Component)]
    #[require(GraphNodes, GraphEdges)]
    pub struct Graph;

    /// The [`Edge`]s of a [`Graph`]
    #[derive(Component, Default)]
    #[relationship_target(relationship = Edge, linked_spawn)]
    pub struct GraphEdges(Vec<Entity>);

    /// The [`Node`]s of a [`Graph`].
    #[derive(Component, Default)]
    #[relationship_target(relationship = Node, linked_spawn)]
    pub struct GraphNodes(Vec<Entity>);

    /// An [`Edge`] within a [`Graph`].
    #[derive(Component)]
    #[relationship(relationship_target = GraphEdges)]
    pub struct Edge(pub Entity);

    /// The input [`Node`] of an [`Edge`].
    #[derive(Component)]
    #[relationship(relationship_target = NodeOutputs)]
    pub struct EdgeInput(pub Entity);

    /// The output [`Node`] of an [`Edge`].
    #[derive(Component)]
    #[relationship(relationship_target = NodeInputs)]
    pub struct EdgeOutput(pub Entity);

    /// A [`Node`] within a [`Graph`].
    #[derive(Component)]
    #[relationship(relationship_target = GraphNodes)]
    #[require(NodeInputs, NodeOutputs)]
    pub struct Node(pub Entity);

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
    GraphEdges,
    GraphNodes,
    Edge,
    EdgeInput,
    EdgeOutput,
    Node,
    NodeInputs,
    NodeOutputs
};

