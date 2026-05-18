//! Graph markers, transformers, and systems





use bevy::prelude::*;
use sentinel_graph::{Edge, Graph, commands::twist_cycle};

mod gates;

pub fn plugin(app: &mut App) {

}

pub fn spawn_default_graph<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
    
    let graph = commands.spawn((
        Name::new("Progression Graph"),
        ProgressionGraph,
        Graph::default()
    )).id();
    
    let start = commands.spawn((
        Name::new("Game Start"),
        GameStart
    )).id();

    let end = commands.spawn((
        Name::new("Game End"),
        GameEnd
    )).id();

    let start_end = commands.spawn((
        Edge::new(start, end, graph),
        ProgressEdge,
        Name::new("Forward Progress")
    )).id();

    let end_start = commands.spawn(Edge::new(end, start, graph)).id();

    twist_cycle(commands, (start_end, end_start));

    commands.entity(graph)
}

/// A label for a progression graph
#[derive(Component)]
pub struct ProgressionGraph;

/// The start of the progression graph
#[derive(Component)]
pub struct GameStart;

/// The end of the progression graph
#[derive(Component)]
pub struct GameEnd;

/// An edge indicating forwards progress
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ProgressEdge;















// fn nodes_behind(world: &World, node: Entity) -> Vec<Entity> {
//     node_inputs(world, node)
//         .iter()
//         .flat_map(Deref::deref)
//         .filter(|input| world.entity(**input).contains::<ProgressEdge>())
//         .flat_map(|edge| edge_input(world, *edge))
//         .collect()
// }







// fn find_cycle_with(world: &World, a: Entity, b: Entity) -> Option<(Entity, Entity)> {
//     let a_b = node_outputs(world, a)
//         .and_then(|outputs| {
//             outputs.iter().find(|edge| {
//                 edge_output(world, **edge).is_some_and(|output| output == b)
//             })
//         });

//     let b_a = node_inputs(world, a)
//         .and_then(|inputs| {
//             inputs.iter().find(|edge| {
//                 edge_input(world, **edge).is_some_and(|input| input == b)
//             })
//         });

//     if let Some(&a_b) = a_b && let Some(&b_a) = b_a {
//         Some((a_b, b_a))
//     } else {
//         None
//     }
// }


// fn is_cycle(world: &World, a_b: Entity, b_a: Entity) -> bool {
//     if let Some(a_one) = edge_input(world, a_b) &&
//         let Some(b_one) = edge_output(world, a_b) &&
//         let Some(b_two) = edge_input(world, b_a) &&
//         let Some(a_two) = edge_output(world, b_a) &&
//         a_one == a_two && b_one == b_two {
//             true
//         } else {
//             false
//         }
// }

// fn spawn_node_between_cycle<'a>(commands: &'a mut Commands, a_b: Entity, b_a: Entity) -> EntityCommands<'a> {

//     let new = commands.spawn_empty().id();

//     commands.queue(move |world: &mut World| {
//         if is_cycle(world, a_b, b_a) {
//             let new_b = world.spawn_empty().id();
//             EntityCloner::build_opt_out(world).clone_entity(a_b, new_b);
//             world.entity_mut(new_b).insert(EdgeInput(new));
//             world.entity_mut(a_b).insert(EdgeOutput(new));

//             let new_a = world.spawn_empty().id();
//             EntityCloner::build_opt_out(world).clone_entity(b_a, new_a);
//             world.entity_mut(new_a).insert(EdgeInput(new));
//             world.entity_mut(b_a).insert(EdgeOutput(new));
//         }
//     });

//     commands.entity(new)
// }





