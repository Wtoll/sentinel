//! The components for constructing graphs

use bevy::{
    ecs::{
        entity::UniqueEntityVec,
        error::CommandWithEntity,
        lifecycle::HookContext,
        relationship::RelationshipHookMode,
        world::DeferredWorld
    },
    prelude::*
};

/// Type alias to disambiguate [`Node`] when also importing [`bevy::prelude`].
pub type GraphNode = Node;

/// A component representing a node in one or more graphs
#[derive(Component, Default)]
#[component(on_replace)]
#[component(on_despawn)]
pub struct Node {
    inputs: UniqueEntityVec,
    outputs: UniqueEntityVec
}

impl Node {
    fn on_replace(mut world: DeferredWorld, context: HookContext) {
        if let HookContext {
            entity,
            relationship_hook_mode: RelationshipHookMode::Run,
            ..
        } = context {
            let (entities, mut commands) = world.entities_and_commands();

            Node::entity_connections(entities.get(entity).unwrap())
                .for_each(|edge| {
                    commands.entity(edge).try_remove::<Edge>();
                });
        }
    }

    fn on_despawn(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
        let (entities, mut commands) = world.entities_and_commands();

        Node::entity_connections(entities.get(entity).unwrap())
            .for_each(|edge| {
                commands.entity(edge).try_despawn();
            });
    }

    fn remove_if_empty(mut entity: EntityWorldMut) {
        if let Some(node) = entity.get::<Node>()
            && node.inputs.is_empty()
            && node.outputs.is_empty() {
                entity.remove::<Node>();
            }
    }

    pub fn has_inputs(&self) -> bool {
        !self.inputs.is_empty()
    }

    pub fn has_outputs(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub fn has_connections(&self) -> bool {
        self.has_inputs() || self.has_outputs()
    }

    pub fn inputs(&self) -> &UniqueEntityVec {
        &self.inputs
    }

    pub fn entity_inputs(entity: EntityRef) -> impl Iterator<Item = Entity> {
        entity
            .get::<Self>()
            .map(|node| node.inputs.clone())
            .unwrap_or_default()
            .into_inner()
            .into_iter()
    }

    pub fn outputs(&self) -> &UniqueEntityVec {
        &self.outputs
    }

    pub fn entity_outputs(entity: EntityRef) -> impl Iterator<Item = Entity> {
        entity
            .get::<Self>()
            .map(|node| node.outputs.clone())
            .unwrap_or_default()
            .into_inner()
            .into_iter()
    }

    pub fn inputs_outputs(&self) -> (&UniqueEntityVec, &UniqueEntityVec) {
        (self.inputs(), self.outputs())
    }

    pub fn entity_inputs_outputs(entity: EntityRef) -> (impl Iterator<Item = Entity>, impl Iterator<Item = Entity>) {
        let (inputs, outputs) = entity.get::<Self>()
            .map(|node| (node.inputs.clone(), node.outputs.clone()))
            .unwrap_or_default();

        (
            inputs.into_inner().into_iter(),
            outputs.into_inner().into_iter()
        )
    }

    pub fn entity_connections(entity: EntityRef) -> impl Iterator<Item = Entity> {
        let (inputs, outputs) = entity.get::<Self>()
            .map(|node| (node.inputs.clone(), node.outputs.clone()))
            .unwrap_or_default();

        inputs.into_inner().into_iter().chain(outputs.into_inner())
    }
}

/// A component representing an edge between two nodes in a graph
#[derive(Component, Clone)]
#[component(on_insert)]
#[component(on_replace)]
pub struct Edge {
    input: Entity,
    output: Entity,
    graph: Entity
}

impl Edge {

    pub fn new(input: Entity, output: Entity, graph: Entity) -> Self {
        Self { input, output, graph }
    }

    fn on_insert(mut world: DeferredWorld, context: HookContext) {
        info!("On insert triggered");
        if let HookContext {
            entity,
            caller,
            relationship_hook_mode: RelationshipHookMode::Run,
            ..
        } = context {
            info!("Passed the hook filter");
            let (entities, mut commands) = world.entities_and_commands();

            let edge = entities.get(entity).unwrap()
                .get::<Self>().unwrap();

            if edge.input == entity {
                warn!("{}The input of edge {entity:?} pointed to itself. This malformed edge has been removed.",
                    caller.map(|location| format!("{location}: ")).unwrap_or_default());
                commands.entity(entity).remove::<Self>();
                return;
            }

            if edge.output == entity {
                warn!("{}The output of edge {entity:?} pointed to itself. This malformed edge has been removed.",
                    caller.map(|location| format!("{location}: ")).unwrap_or_default());
                commands.entity(entity).remove::<Self>();
                return;
            }

            if edge.graph == entity {
                warn!("{}The parent graph of edge {entity:?} pointed to itself. This malformed edge has been removed.",
                    caller.map(|location| format!("{location}: ")).unwrap_or_default());
                commands.entity(entity).remove::<Self>();
                return;
            }

            let (input, output, graph) = (edge.input, edge.output, edge.graph);

            if let Ok(input_target) = entities.get(input) {
                if !input_target.contains::<Self>() {
                    commands.get_entity(input).unwrap()
                        .entry::<Node>()
                        .or_default()
                        .and_modify(move |mut node| {
                            if !node.outputs.contains(&entity) {
                                // SAFETY: We check before insertion to ensure uniqueness
                                unsafe { node.outputs.push(entity); }
                            }
                        });
                } else {
                    warn!("{}The input of edge {entity:?} pointed to another edge {}. This malformed edge has been removed.",
                        caller.map(|location| format!("{location}: ")).unwrap_or_default(),
                        input);
                    commands.entity(entity).remove::<Self>();
                    return;
                }
            } else {
                warn!("{}The input of edge {entity:?} pointed to an entity that does not exist. This malformed edge has been removed.",
                    caller.map(|location| format!("{location}: ")).unwrap_or_default());
                commands.entity(entity).remove::<Self>();
                return;
            }

            if let Ok(output_target) = entities.get(output) {
                if !output_target.contains::<Self>() {
                    commands.get_entity(output).unwrap()
                        .entry::<Node>()
                        .or_default()
                        .and_modify(move |mut node| {
                            if !node.inputs.contains(&entity) {
                                // SAFETY: We check before insertion to ensure uniqueness
                                unsafe { node.inputs.push(entity); }
                            }
                        });
                } else {
                    warn!("{}The output of edge {entity:?} pointed to another edge {}. This malformed edge has been removed.",
                        caller.map(|location| format!("{location}: ")).unwrap_or_default(),
                        output);
                    commands.entity(entity).remove::<Self>();
                    return;
                }
            } else {
                warn!("{}The output of edge {entity:?} pointed to an entity that does not exist. This malformed edge has been removed.",
                    caller.map(|location| format!("{location}: ")).unwrap_or_default());
                commands.entity(entity).remove::<Self>();
                return;
            }

            if let Ok(graph_target) = entities.get(graph) {
                if !graph_target.contains::<Self>() {
                    commands.get_entity(graph).unwrap()
                        .entry::<Graph>()
                        .or_default()
                        .and_modify(move |mut graph| {
                            if !graph.edges.contains(&entity) {
                                // SAFETY: We check before insertion to ensure uniqueness
                                unsafe { graph.edges.push(entity); }
                            }
                        });
                } else {
                    warn!("{}The parent graph of edge {entity:?} pointed to another edge {}. This malformed edge has been removed.",
                        caller.map(|location| format!("{location}: ")).unwrap_or_default(),
                        graph);
                    commands.entity(entity).remove::<Self>();
                    return;
                }
            } else {
                warn!("{}The parent graph of edge {entity:?} pointed to an entity that does not exist. This malformed edge has been removed.",
                    caller.map(|location| format!("{location}: ")).unwrap_or_default());
                commands.entity(entity).remove::<Self>();
                return;
            }

        }
    }

    fn on_replace(mut world: DeferredWorld, context: HookContext) {
        if let HookContext {
            entity,
            relationship_hook_mode: RelationshipHookMode::Run,
            ..
        } = context {
            let (mut entities, mut commands) = world.entities_and_commands();

            let edge = entities.get(entity).unwrap()
                .get::<Self>().unwrap();

            let (input, output, graph) = (edge.input, edge.output, edge.graph);

            if let Ok(mut input_target) = entities.get_mut(input)
                && let Some(mut node) = input_target.get_mut::<Node>() {
                    node.outputs.retain(|item| *item != entity);
                    if node.inputs.is_empty() && node.outputs.is_empty() {
                        commands.queue(Node::remove_if_empty.with_entity(input));
                    }
                }
            
            if let Ok(mut output_target) = entities.get_mut(output)
                && let Some(mut node) = output_target.get_mut::<Node>() {
                    node.inputs.retain(|item| *item != entity);
                    if node.inputs.is_empty() && node.outputs.is_empty() {
                        commands.queue(Node::remove_if_empty.with_entity(output));
                    }
                }

            if let Ok(mut graph_target) = entities.get_mut(graph)
                && let Some(mut graph_c) = graph_target.get_mut::<Graph>() {
                    graph_c.edges.retain(|item| *item != entity);
                    if graph_c.edges.is_empty() {
                        commands.queue(Graph::remove_if_empty.with_entity(graph));
                    }
                }
        }
    }

    pub fn input(&self) -> Entity {
        self.input
    }

    pub fn entity_input(entity: EntityRef) -> Option<Entity> {
        entity.get::<Self>().map(|edge| edge.input)
    }

    pub fn output(&self) -> Entity {
        self.output
    }

    pub fn entity_output(entity: EntityRef) -> Option<Entity> {
        entity.get::<Self>().map(|edge| edge.output)
    }

    pub fn graph(&self) -> Entity {
        self.graph
    }

    pub fn entity_graph(entity: EntityRef) -> Option<Entity> {
        entity.get::<Self>().map(|edge| edge.graph)
    }

    pub fn is_cycle(edges: (&Self, &Self)) -> bool {
        edges.0.graph == edges.1.graph && edges.0.output == edges.1.input && edges.1.output == edges.0.input
    }
}

/// A component representing a graph of nodes connected by edges
#[derive(Component, Default)]
#[component(on_replace)]
#[component(on_despawn)]
pub struct Graph {
    edges: UniqueEntityVec
}

impl Graph {
    fn on_replace(mut world: DeferredWorld, context: HookContext) {
        if let HookContext {
            entity,
            relationship_hook_mode: RelationshipHookMode::Run,
            ..
        } = context {
            let (entities, mut commands) = world.entities_and_commands();

            let graph = entities.get(entity).unwrap()
                .get::<Self>().unwrap();

            graph.edges.iter()
                .for_each(|edge| {
                    commands.entity(*edge).try_remove::<Edge>();
                });
        }
    }

    fn on_despawn(mut world: DeferredWorld, context: HookContext) {
        let HookContext { entity, .. } = context;
        
        let (entities, mut commands) = world.entities_and_commands();

        let graph = entities.get(entity).unwrap()
            .get::<Self>().unwrap();

        graph.edges.iter()
            .for_each(|edge| {
                commands.entity(*edge).try_despawn();
            });
    }

    fn remove_if_empty(mut entity: EntityWorldMut) {
        if let Some(graph) = entity.get::<Graph>()
            && graph.edges.is_empty() {
                entity.remove::<Graph>();
            }
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn edges(&self) -> &UniqueEntityVec {
        &self.edges
    }

    pub fn entity_edges(entity: EntityRef) -> impl Iterator<Item = Entity> {
        entity
            .get::<Self>()
            .map(|graph| graph.edges.clone())
            .unwrap_or_default()
            .into_inner()
            .into_iter()
    }
}