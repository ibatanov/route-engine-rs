use route_engine_rs::errors::{DijkstraError, GraphError};

#[test]
fn graph_error_display_is_readable() {
    let err = GraphError::FromNodeOutOfBounds {
        index: 5,
        node_count: 3,
    };
    let msg = err.to_string();
    assert!(msg.contains("from node out of bounds"));
    assert!(msg.contains("5 >= 3"));
}

#[test]
fn dijkstra_error_display_is_readable() {
    let err = DijkstraError::TargetNodeOutOfBounds {
        index: 9,
        node_count: 4,
    };
    let msg = err.to_string();
    assert!(msg.contains("target node out of bounds"));
    assert!(msg.contains("9 >= 4"));
}
