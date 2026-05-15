//! A graph data structure implementation for Bevy

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
        NodeOutputs,
        GraphNode
    };

    pub use super::{
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

mod core;

pub use core::{
    Graph,
    Edge,
    EdgeGraph,
    EdgeInput,
    EdgeOutput,
    Node,
    NodeInputs,
    NodeOutputs,
    GraphNode
};

pub mod util;

pub use util::{
    parent_graph,
    edge_graph,
    edge_input,
    edge_output,
    node_inputs,
    node_outputs,
    in_graph
};

pub mod commands;