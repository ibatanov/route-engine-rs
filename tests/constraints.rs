use route_engine_rs::algorithms::{shortest_path, shortest_path_with_constraint};
use route_engine_rs::constraints::allow::AllowAll;
use route_engine_rs::constraints::{EdgeContext, PathConstraint};
use route_engine_rs::graph::{Graph, NodeId};
use route_engine_rs::strategies::by_cost::ByCost;

#[derive(Clone, Copy, Debug)]
struct City {
    is_transit: bool,
}

#[derive(Clone, Copy, Debug)]
struct Road {
    cost: u64,
}

struct TransitConstraint;

impl PathConstraint<City, Road, u64> for TransitConstraint {
    fn allow_edge(
        &self,
        ctx: &EdgeContext<'_, City, Road>,
        _prev_state: &u64,
        _next_state: &u64,
    ) -> bool {
        if ctx.from == ctx.source || ctx.from == ctx.target {
            return true;
        }
        ctx.from_node.is_transit
    }
}

#[test]
fn allow_all_constraint_matches_shortest_path() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, 4).unwrap();
    graph.add_edge(b, c, 1).unwrap();
    graph.add_edge(a, c, 10).unwrap();

    let strategy = ByCost::new(|edge: &u64| *edge);
    let base = shortest_path(&graph, a, c, &strategy).unwrap();
    let with_constraint =
        shortest_path_with_constraint(&graph, a, c, &strategy, &AllowAll).unwrap();

    assert_eq!(
        base.as_ref().map(|p| p.state),
        with_constraint.as_ref().map(|p| p.state)
    );
    assert_eq!(
        base.as_ref()
            .map(|p| p.nodes.iter().map(|n| n.index()).collect::<Vec<_>>()),
        with_constraint
            .as_ref()
            .map(|p| p.nodes.iter().map(|n| n.index()).collect::<Vec<_>>())
    );
}

#[test]
fn transit_constraint_blocks_non_transit_pass_through() {
    let mut graph: Graph<City, Road> = Graph::new();
    let source = graph.add_node(City { is_transit: true });
    let blocked_mid = graph.add_node(City { is_transit: false });
    let transit_mid = graph.add_node(City { is_transit: true });
    let target = graph.add_node(City { is_transit: true });

    graph
        .add_edge(source, blocked_mid, Road { cost: 1 })
        .unwrap();
    graph
        .add_edge(blocked_mid, target, Road { cost: 1 })
        .unwrap();
    graph
        .add_edge(source, transit_mid, Road { cost: 3 })
        .unwrap();
    graph
        .add_edge(transit_mid, target, Road { cost: 3 })
        .unwrap();

    let strategy = ByCost::new(|edge: &Road| edge.cost);
    let constraint = TransitConstraint;

    let result = shortest_path_with_constraint(&graph, source, target, &strategy, &constraint)
        .unwrap()
        .expect("path must exist via transit node");

    assert_eq!(result.state, 6);
    let path: Vec<usize> = result.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(
        path,
        vec![source.index(), transit_mid.index(), target.index()]
    );
}

#[test]
fn transit_constraint_allows_reaching_non_transit_target() {
    let mut graph: Graph<City, Road> = Graph::new();
    let source = graph.add_node(City { is_transit: true });
    let non_transit_target = graph.add_node(City { is_transit: false });

    graph
        .add_edge(source, non_transit_target, Road { cost: 7 })
        .unwrap();

    let strategy = ByCost::new(|edge: &Road| edge.cost);
    let constraint = TransitConstraint;

    let result =
        shortest_path_with_constraint(&graph, source, non_transit_target, &strategy, &constraint)
            .unwrap()
            .expect("direct path must be allowed");

    assert_eq!(result.state, 7);
    let path: Vec<usize> = result.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(path, vec![source.index(), non_transit_target.index()]);
}
