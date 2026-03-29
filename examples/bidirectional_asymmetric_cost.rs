use std::error::Error;

use route_engine_rs::algorithms::{PathResult, shortest_path};
use route_engine_rs::graph::{Graph, NodeId};
use route_engine_rs::strategies::by_cost::ByCost;

fn format_path(graph: &Graph<&'static str, u64>, path: &PathResult<u64>) -> String {
    path
        .nodes
        .iter()
        .map(|id| graph.node(*id).copied().unwrap_or("<unknown>"))
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn print_direction(
    graph: &Graph<&'static str, u64>,
    from: NodeId,
    to: NodeId,
    strategy: &ByCost<impl Fn(&u64) -> u64>,
) {
    let from_name = graph.node(from).copied().unwrap_or("?");
    let to_name = graph.node(to).copied().unwrap_or("?");

    println!("Направление {from_name} -> {to_name}:");
    match shortest_path(graph, from, to, strategy) {
        Ok(Some(path)) => {
            println!("  Маршрут: {}", format_path(graph, &path));
            println!("  Стоимость: {}", path.state);
        }
        Ok(None) => println!("  Путь не найден"),
        Err(err) => println!("  Ошибка поиска: {err}"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph: Graph<&str, u64> = Graph::new();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    // Два направления между A и B имеют разную стоимость.
    graph.add_edge(a, b, 10)?;
    graph.add_edge(b, a, 1)?;

    // В направлении A -> B выгоднее идти через C.
    graph.add_edge(a, c, 3)?;
    graph.add_edge(c, b, 3)?;

    let strategy = ByCost::new(|edge: &u64| *edge);

    print_direction(&graph, a, b, &strategy);
    print_direction(&graph, b, a, &strategy);

    Ok(())
}
