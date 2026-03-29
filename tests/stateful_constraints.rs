use route_engine_rs::algorithms::{shortest_path, shortest_path_with_constraint};
use route_engine_rs::constraints::{EdgeContext, PathConstraint};
use route_engine_rs::graph::{Graph, NodeId};
use route_engine_rs::strategies::PathStrategy;
use route_engine_rs::strategies::by_cost::ByCost;

struct MaxTotalCostConstraint {
    max_total_cost: u64,
}

impl<N> PathConstraint<N, u64, u64> for MaxTotalCostConstraint {
    fn allow_edge(
        &self,
        _ctx: &EdgeContext<'_, N, u64>,
        _prev_state: &u64,
        next_state: &u64,
    ) -> bool {
        *next_state <= self.max_total_cost
    }
}

#[test]
fn stateful_constraint_limits_total_cost() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, 4).unwrap();
    graph.add_edge(b, c, 4).unwrap();
    graph.add_edge(a, c, 10).unwrap();

    let strategy = ByCost::new(|edge: &u64| *edge);
    let limit = MaxTotalCostConstraint { max_total_cost: 8 };

    let result = shortest_path_with_constraint(&graph, a, c, &strategy, &limit)
        .unwrap()
        .expect("path within cost limit must exist");

    assert_eq!(result.state, 8);
    let path: Vec<usize> = result.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(path, vec![a.index(), b.index(), c.index()]);
}

#[test]
fn stateful_constraint_returns_none_when_limit_is_too_strict() {
    let mut graph: Graph<&str, u64> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, 4).unwrap();
    graph.add_edge(b, c, 4).unwrap();
    graph.add_edge(a, c, 10).unwrap();

    let strategy = ByCost::new(|edge: &u64| *edge);
    let limit = MaxTotalCostConstraint { max_total_cost: 7 };

    let result = shortest_path_with_constraint(&graph, a, c, &strategy, &limit).unwrap();
    assert!(result.is_none());
}

#[derive(Clone, Copy, Debug)]
struct Road {
    price: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct CostAndHops {
    total_price: u64,
    hops: u64,
}

struct ByPriceWithHops;

impl PathStrategy<Road> for ByPriceWithHops {
    type State = CostAndHops;
    type Key = u64;

    fn start_state(&self) -> Self::State {
        CostAndHops::default()
    }

    fn next_state(&self, prev: &Self::State, edge: &Road) -> Self::State {
        CostAndHops {
            total_price: prev.total_price + edge.price,
            hops: prev.hops + 1,
        }
    }

    fn key(&self, state: &Self::State) -> Self::Key {
        state.total_price
    }
}

struct MaxHopsConstraint {
    max_hops: u64,
}

impl<N> PathConstraint<N, Road, CostAndHops> for MaxHopsConstraint {
    fn allow_edge(
        &self,
        _ctx: &EdgeContext<'_, N, Road>,
        _prev_state: &CostAndHops,
        next_state: &CostAndHops,
    ) -> bool {
        next_state.hops <= self.max_hops
    }
}

#[test]
fn stateful_constraint_can_limit_hops() {
    let mut graph: Graph<&str, Road> = Graph::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let e = graph.add_node("E");
    let d = graph.add_node("D");

    graph.add_edge(a, b, Road { price: 5 }).unwrap();
    graph.add_edge(b, d, Road { price: 5 }).unwrap();

    graph.add_edge(a, c, Road { price: 1 }).unwrap();
    graph.add_edge(c, e, Road { price: 1 }).unwrap();
    graph.add_edge(e, d, Road { price: 1 }).unwrap();

    let strategy = ByPriceWithHops;
    let baseline = shortest_path(&graph, a, d, &strategy)
        .unwrap()
        .expect("unconstrained path must exist");
    assert_eq!(baseline.state.total_price, 3);
    assert_eq!(baseline.state.hops, 3);

    let limit = MaxHopsConstraint { max_hops: 2 };
    let constrained = shortest_path_with_constraint(&graph, a, d, &strategy, &limit)
        .unwrap()
        .expect("path with hop limit must exist");

    assert_eq!(constrained.state.total_price, 10);
    assert_eq!(constrained.state.hops, 2);
    let path: Vec<usize> = constrained.nodes.into_iter().map(NodeId::index).collect();
    assert_eq!(path, vec![a.index(), b.index(), d.index()]);
}
