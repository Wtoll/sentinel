//! [`Command`]s and extensions to [`Commands`] for interacting with
//! [`Graph`]s, [`Node`]s, and [`Edge`]s.

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
    /// # Example
    /// 
    /// ```
    /// use bevy::prelude::*;
    /// use sentinel_test::prelude::*;
    /// use sentinel_graph::prelude::*;
    /// 
    /// #[derive(Component)]
    /// struct TargetEdge;
    /// 
    /// SteppableApp::default()
    ///     .step(|mut commands: Commands| {
    ///         let node_a = commands.spawn(Name::new("Node A")).id();
    ///         let (node_b, edge_a_b) = commands.spawn_connected_from(node_a);
    ///         commands.entity(edge_a_b).insert(TargetEdge);
    ///         commands.entity(node_b).insert(Name::new("Node B"));
    ///     })
    ///     .step(|world: &mut World| {
    ///         if let Ok((edge_a_b, _)) = world.query::<(Entity, &TargetEdge)>().single(world) {
    ///             assert_eq!(
    ///                 edge_input(world, edge_a_b)
    ///                     .and_then(|input| world.get::<Name>(input))
    ///                         .unwrap()
    ///                         .as_str(),
    ///                 "Node A"
    ///             );
    ///             assert_eq!(
    ///                 edge_output(world, edge_a_b)
    ///                     .and_then(|output| world.get::<Name>(output))
    ///                         .unwrap()
    ///                         .as_str(),
    ///                 "Node B"
    ///             );
    ///         }
    ///     });
    /// ```
    /// 
    /// See [`spawn_connected_to`](Self::spawn_connected_to) and
    /// [`spawn_cyclic_with`](Self::spawn_cyclic_with).
    /// 
    fn spawn_connected_from(&mut self, from: Entity) -> (Entity, Entity);

    /// Spawns a new [`Node`] with a connection to the given [`Node`].
    /// 
    /// Returns a tuple containing the new [`Node`] and the new [`Edge`]
    /// between them.
    /// 
    /// # Example
    /// 
    /// ```
    /// use bevy::prelude::*;
    /// use sentinel_test::prelude::*;
    /// use sentinel_graph::prelude::*;
    /// 
    /// #[derive(Component)]
    /// struct TargetEdge;
    /// 
    /// SteppableApp::default()
    ///     .step(|mut commands: Commands| {
    ///         let node_a = commands.spawn(Name::new("Node A")).id();
    ///         let (node_b, edge_b_a) = commands.spawn_connected_to(node_a);
    ///         commands.entity(edge_b_a).insert(TargetEdge);
    ///         commands.entity(node_b).insert(Name::new("Node B"));
    ///     })
    ///     .step(|world: &mut World| {
    ///         if let Ok((edge_b_a, _)) = world.query::<(Entity, &TargetEdge)>().single(world) {
    ///             assert_eq!(
    ///                 edge_input(world, edge_b_a)
    ///                     .and_then(|input| world.get::<Name>(input))
    ///                         .unwrap()
    ///                         .as_str(),
    ///                 "Node B"
    ///             );
    ///             assert_eq!(
    ///                 edge_output(world, edge_b_a)
    ///                     .and_then(|output| world.get::<Name>(output))
    ///                         .unwrap()
    ///                         .as_str(),
    ///                 "Node A"
    ///             );
    ///         }
    ///     });
    /// ```
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

#[cfg(test)]
mod test {
    use bevy::ecs::system::RunSystemOnce;
    use sentinel_test::prelude::*;
    use crate::prelude::*;
    use bevy::prelude::*;






    #[test]
    fn test_spawn_connected_from() {

        #[derive(Component)]
        struct TargetEdge;

        test_app(|app: &mut App| {
            app
                .add_systems(Startup, |world: &mut World| {
                    let mut commands = world.commands();

                    let node_a = commands.spawn(Name::new("Node A")).id();

                    let (node_b, edge_a_b) = commands.spawn_connected_from(node_a);

                    commands.entity(edge_a_b).insert(TargetEdge);

                    commands.entity(node_b).insert(Name::new("Node B"));
                })
                .add_systems(Update, |world: &mut World| {
                    if let Ok((edge_a_b, _)) = world.query::<(Entity, &TargetEdge)>().single(world) {
                        assert_eq!(
                            edge_input(world, edge_a_b)
                                .and_then(|input| world.get::<Name>(input))
                                .unwrap()
                                .as_str(),
                            "Node A"
                        );
                        assert_eq!(
                            edge_output(world, edge_a_b)
                                .and_then(|output| world.get::<Name>(output))
                                .unwrap()
                                .as_str(),
                            "Node B"
                        );
                    }
                })
                .add_systems(Update, cycles(5).pipe(then_exit));
        })
    }

    #[test]
    fn test_spawn_connected_to() {

        #[derive(Component)]
        struct TargetEdge;

        test_app(|app: &mut App| {
            app
                .add_systems(Startup, |world: &mut World| {
                    let mut commands = world.commands();

                    let node_a = commands.spawn(Name::new("Node A")).id();

                    let (node_b, edge_b_a) = commands.spawn_connected_to(node_a);

                    commands.entity(edge_b_a).insert(TargetEdge);

                    commands.entity(node_b).insert(Name::new("Node B"));
                })
                .add_systems(Update, |world: &mut World| {
                    if let Ok((edge_b_a, _)) = world.query::<(Entity, &TargetEdge)>().single(world) {
                        assert_eq!(
                            edge_input(world, edge_b_a)
                                .and_then(|input| world.get::<Name>(input))
                                .unwrap()
                                .as_str(),
                            "Node B"
                        );
                        assert_eq!(
                            edge_output(world, edge_b_a)
                                .and_then(|output| world.get::<Name>(output))
                                .unwrap()
                                .as_str(),
                            "Node A"
                        );
                    }
                })
                .add_systems(Update, cycles(5).pipe(then_exit));
        })
    }

    #[test]
    fn test_spawn_cyclic() {

        #[derive(Component)]
        struct TargetEdge;

        test_app(|app: &mut App| {
            app
                .add_systems(Startup, |world: &mut World| {
                    let mut commands = world.commands();

                    let node_a = commands.spawn(Name::new("Node A")).id();

                    let (node_b, edge_a_b, _edge_b_a) = commands.spawn_cyclic_with(node_a);

                    commands.entity(edge_a_b).insert(TargetEdge);

                    commands.entity(node_b).insert(Name::new("Node B"));
                })
                .add_systems(Update, |world: &mut World| {
                    if let Ok((edge_a_b, _)) = world.query::<(Entity, &TargetEdge)>().single(world) {
                        assert_eq!(
                            edge_input(world, edge_a_b)
                                .and_then(|input| world.get::<Name>(input))
                                .unwrap()
                                .as_str(),
                            "Node A"
                        );
                        assert_eq!(
                            edge_output(world, edge_a_b)
                                .and_then(|output| world.get::<Name>(output))
                                .unwrap()
                                .as_str(),
                            "Node B"
                        );
                    }
                })
                .add_systems(Update, cycles(5).pipe(then_exit));
        })
    }
}