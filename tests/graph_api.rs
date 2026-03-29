use route_engine_rs::errors::GraphError;
use route_engine_rs::graph::{Graph, NodeId};

#[test]
fn graph_new_is_empty() {
    let graph: Graph<&str, u64> = Graph::new();
    assert!(graph.is_empty());
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn graph_add_node_returns_sequential_ids() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    assert_eq!(a.index(), 0);
    assert_eq!(b.index(), 1);
    assert_eq!(c.index(), 2);
}

#[test]
fn graph_add_edge_success_increments_edge_count() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");

    graph.add_edge(a, b, 42).expect("edge must be added");

    assert_eq!(graph.edge_count(), 1);
    let neighbors = graph.neighbors(a).expect("neighbors must exist");
    assert_eq!(neighbors.len(), 1);
    assert_eq!(neighbors[0].0, b);
    assert_eq!(neighbors[0].1, 42);
}

#[test]
fn graph_add_edge_invalid_from_returns_error() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let b = graph.add_node("B");

    let err = graph
        .add_edge(NodeId::new(99), b, 1)
        .expect_err("invalid from must fail");

    assert_eq!(
        err,
        GraphError::FromNodeOutOfBounds {
            index: 99,
            node_count: 1,
        }
    );
}

#[test]
fn graph_add_edge_invalid_to_returns_error() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");

    let err = graph
        .add_edge(a, NodeId::new(77), 1)
        .expect_err("invalid to must fail");

    assert_eq!(
        err,
        GraphError::ToNodeOutOfBounds {
            index: 77,
            node_count: 1,
        }
    );
}
