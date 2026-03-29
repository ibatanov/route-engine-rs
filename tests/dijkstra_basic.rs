use route_engine_rs::algorithms::shortest_path;
use route_engine_rs::errors::DijkstraError;
use route_engine_rs::graph::{Graph, NodeId};
use route_engine_rs::strategies::by_cost::ByCost;

#[test]
fn dijkstra_invalid_source_returns_error() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let strategy = ByCost::new(|edge: &u64| *edge);

    let err = shortest_path(&graph, NodeId::new(10), a, &strategy).expect_err("must fail");
    assert_eq!(
        err,
        DijkstraError::SourceNodeOutOfBounds {
            index: 10,
            node_count: 1,
        }
    );
}

#[test]
fn dijkstra_invalid_target_returns_error() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let strategy = ByCost::new(|edge: &u64| *edge);

    let err = shortest_path(&graph, a, NodeId::new(11), &strategy).expect_err("must fail");
    assert_eq!(
        err,
        DijkstraError::TargetNodeOutOfBounds {
            index: 11,
            node_count: 1,
        }
    );
}

#[test]
fn dijkstra_unreachable_returns_none() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let strategy = ByCost::new(|edge: &u64| *edge);

    let result = shortest_path(&graph, a, b, &strategy).expect("must be valid call");
    assert!(result.is_none());
}

#[test]
fn dijkstra_from_equals_to_returns_zero_state() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let strategy = ByCost::new(|edge: &u64| *edge);

    let result = shortest_path(&graph, a, a, &strategy)
        .expect("must be valid call")
        .expect("path to itself must exist");

    assert_eq!(result.state, 0);
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0], a);
}

#[test]
fn by_cost_picks_min_total_cost_path() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b, 5).unwrap();
    graph.add_edge(b, d, 5).unwrap();
    graph.add_edge(a, c, 2).unwrap();
    graph.add_edge(c, d, 3).unwrap();

    let strategy = ByCost::new(|edge: &u64| *edge);
    let result = shortest_path(&graph, a, d, &strategy)
        .expect("must be valid call")
        .expect("path must exist");

    assert_eq!(result.state, 5);
    let ids: Vec<usize> = result.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(ids, vec![0, 2, 3]);
}

#[test]
fn by_cost_handles_bidirectional_graph_with_different_costs() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    // Два направления между A и B имеют разные стоимости.
    graph.add_edge(a, b, 10).unwrap();
    graph.add_edge(b, a, 1).unwrap();

    // Дополнительная дуга показывает, что в одном направлении
    // выгоднее обход, а в обратном — прямой путь.
    graph.add_edge(a, c, 3).unwrap();
    graph.add_edge(c, b, 3).unwrap();

    let strategy = ByCost::new(|edge: &u64| *edge);

    let a_to_b = shortest_path(&graph, a, b, &strategy)
        .expect("must be valid call")
        .expect("path A->B must exist");
    assert_eq!(a_to_b.state, 6);
    let a_to_b_path: Vec<usize> = a_to_b.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(a_to_b_path, vec![a.index(), c.index(), b.index()]);

    let b_to_a = shortest_path(&graph, b, a, &strategy)
        .expect("must be valid call")
        .expect("path B->A must exist");
    assert_eq!(b_to_a.state, 1);
    let b_to_a_path: Vec<usize> = b_to_a.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(b_to_a_path, vec![b.index(), a.index()]);
}
