use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphError {
    GraphInvariantBroken {
        node_count: usize,
        adjacency_count: usize,
    },
    FromNodeOutOfBounds {
        index: usize,
        node_count: usize,
    },
    ToNodeOutOfBounds {
        index: usize,
        node_count: usize,
    },
}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GraphInvariantBroken {
                node_count,
                adjacency_count,
            } => write!(
                f,
                "graph invariant broken: nodes={}, adjacency_lists={}",
                node_count, adjacency_count
            ),
            Self::FromNodeOutOfBounds { index, node_count } => {
                write!(f, "from node out of bounds: {} >= {}", index, node_count)
            }
            Self::ToNodeOutOfBounds { index, node_count } => {
                write!(f, "to node out of bounds: {} >= {}", index, node_count)
            }
        }
    }
}

impl Error for GraphError {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DijkstraError {
    SourceNodeOutOfBounds { index: usize, node_count: usize },
    TargetNodeOutOfBounds { index: usize, node_count: usize },
}

impl Display for DijkstraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceNodeOutOfBounds { index, node_count } => {
                write!(f, "source node out of bounds: {} >= {}", index, node_count)
            }
            Self::TargetNodeOutOfBounds { index, node_count } => {
                write!(f, "target node out of bounds: {} >= {}", index, node_count)
            }
        }
    }
}

impl Error for DijkstraError {}
