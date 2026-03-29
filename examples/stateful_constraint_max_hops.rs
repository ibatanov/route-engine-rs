use std::error::Error;

use route_engine_rs::algorithms::shortest_path_with_constraint;
use route_engine_rs::constraints::{EdgeContext, PathConstraint};
use route_engine_rs::graph::Graph;
use route_engine_rs::strategies::PathStrategy;

#[derive(Clone, Copy, Debug)]
struct Leg {
    price_rub: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RouteState {
    total_price_rub: u64,
    hops: u64,
}

struct ByPriceWithHops;

impl PathStrategy<Leg> for ByPriceWithHops {
    type State = RouteState;
    type Key = u64;

    fn start_state(&self) -> Self::State {
        RouteState::default()
    }

    fn next_state(&self, prev: &Self::State, edge: &Leg) -> Self::State {
        RouteState {
            total_price_rub: prev.total_price_rub + edge.price_rub,
            hops: prev.hops + 1,
        }
    }

    fn key(&self, state: &Self::State) -> Self::Key {
        state.total_price_rub
    }
}

struct MaxHopsConstraint {
    max_hops: u64,
}

impl<N> PathConstraint<N, Leg, RouteState> for MaxHopsConstraint {
    fn allow_edge(
        &self,
        _ctx: &EdgeContext<'_, N, Leg>,
        _prev_state: &RouteState,
        next_state: &RouteState,
    ) -> bool {
        next_state.hops <= self.max_hops
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph: Graph<&str, Leg> = Graph::new();
    let supplier_wh = graph.add_node("Склад поставщика");
    let transit_1 = graph.add_node("Транзит 1");
    let transit_2 = graph.add_node("Транзит 2");
    let pickup = graph.add_node("ПВЗ клиента");

    graph.add_edge(supplier_wh, transit_1, Leg { price_rub: 3_000 })?;
    graph.add_edge(transit_1, transit_2, Leg { price_rub: 2_000 })?;
    graph.add_edge(transit_2, pickup, Leg { price_rub: 2_000 })?;

    graph.add_edge(supplier_wh, pickup, Leg { price_rub: 9_000 })?;

    let strategy = ByPriceWithHops;
    let constraint = MaxHopsConstraint { max_hops: 2 };
    let result =
        shortest_path_with_constraint(&graph, supplier_wh, pickup, &strategy, &constraint)?;

    match result {
        Some(path) => {
            println!("Итоговая цена: {} ₽", path.state.total_price_rub);
            println!("Число плеч: {}", path.state.hops);
            println!("Узлов в маршруте: {}", path.nodes.len());
        }
        None => println!("Маршрут не найден"),
    }

    Ok(())
}
