use route_engine_rs::algorithms::shortest_path;
use route_engine_rs::graph::{Graph, NodeId};
use route_engine_rs::strategies::by_two_costs::ByTwoCosts;

#[derive(Clone, Copy, Debug)]
struct Edge {
    primary: u64,
    secondary: u64,
}

#[test]
fn by_two_costs_primary_then_secondary_tie_breaks_correctly() {
    let mut graph: Graph<&str, Edge> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph
        .add_edge(
            a,
            b,
            Edge {
                primary: 10,
                secondary: 5,
            },
        )
        .unwrap();
    graph
        .add_edge(
            a,
            c,
            Edge {
                primary: 10,
                secondary: 1,
            },
        )
        .unwrap();
    graph
        .add_edge(
            b,
            d,
            Edge {
                primary: 0,
                secondary: 0,
            },
        )
        .unwrap();
    graph
        .add_edge(
            c,
            d,
            Edge {
                primary: 0,
                secondary: 0,
            },
        )
        .unwrap();

    let strategy = ByTwoCosts::primary_then_secondary(
        |edge: &Edge| edge.primary,
        |edge: &Edge| edge.secondary,
    );
    let result = shortest_path(&graph, a, d, &strategy)
        .unwrap()
        .expect("path must exist");

    assert_eq!(result.state.primary_cost, 10);
    assert_eq!(result.state.secondary_cost, 1);
    let path: Vec<usize> = result.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(path, vec![a.index(), c.index(), d.index()]);
}

#[test]
fn by_two_costs_secondary_then_primary_tie_breaks_correctly() {
    let mut graph: Graph<&str, Edge> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph
        .add_edge(
            a,
            b,
            Edge {
                primary: 1,
                secondary: 10,
            },
        )
        .unwrap();
    graph
        .add_edge(
            a,
            c,
            Edge {
                primary: 2,
                secondary: 5,
            },
        )
        .unwrap();
    graph
        .add_edge(
            b,
            d,
            Edge {
                primary: 0,
                secondary: 0,
            },
        )
        .unwrap();
    graph
        .add_edge(
            c,
            d,
            Edge {
                primary: 0,
                secondary: 0,
            },
        )
        .unwrap();

    let strategy = ByTwoCosts::secondary_then_primary(
        |edge: &Edge| edge.primary,
        |edge: &Edge| edge.secondary,
    );
    let result = shortest_path(&graph, a, d, &strategy)
        .unwrap()
        .expect("path must exist");

    assert_eq!(result.state.primary_cost, 2);
    assert_eq!(result.state.secondary_cost, 5);
    let path: Vec<usize> = result.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(path, vec![a.index(), c.index(), d.index()]);
}
