use std::error::Error;

use route_engine_rs::algorithms::{PathResult, shortest_path_with_constraint};
use route_engine_rs::constraints::{EdgeContext, PathConstraint};
use route_engine_rs::graph::Graph;
use route_engine_rs::graph::NodeId;
use route_engine_rs::strategies::by_cost::ByCost;

#[derive(Clone, Copy, Debug)]
struct Road {
    price_rub: u64,
    lead_time_days: u64,
}

#[derive(Clone, Copy, Debug)]
struct City {
    name: &'static str,
    is_transit: bool,
}

/// Через нетранзитный город нельзя продолжать маршрут,
/// но можно завершить маршрут в нем или стартовать из него.
struct TransitConstraint;

impl PathConstraint<City, Road> for TransitConstraint {
    fn allow_edge(&self, ctx: &EdgeContext<'_, City, Road>) -> bool {
        if ctx.from == ctx.source || ctx.from == ctx.target {
            return true;
        }
        ctx.from_node.is_transit
    }
}

fn print_path(title: &str, graph: &Graph<City, Road>, path: Option<PathResult<u64>>) {
    println!("{title}");
    match path {
        Some(path) => {
            let route = path
                .nodes
                .iter()
                .map(|id| graph.node(*id).map(|city| city.name).unwrap_or("<unknown>"))
                .collect::<Vec<_>>()
                .join(" -> ");

            println!("Маршрут: {}", route);
            println!("Итоговая стоимость: {} ₽", path.state);
            println!(
                "Срок поставки: {} дн.",
                total_lead_time_days(graph, &path.nodes)
            );
        }
        None => {
            println!("Путь не найден");
        }
    }
}

fn total_lead_time_days(graph: &Graph<City, Road>, nodes: &[NodeId]) -> u64 {
    nodes
        .windows(2)
        .map(|pair| {
            let from = pair[0];
            let to = pair[1];
            graph
                .neighbors(from)
                .and_then(|neighbors| {
                    neighbors.iter().find_map(|(next, edge)| {
                        if *next == to {
                            Some(edge.lead_time_days)
                        } else {
                            None
                        }
                    })
                })
                .unwrap_or(0)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph: Graph<City, Road> = Graph::new();

    let moscow = graph.add_node(City {
        name: "Москва",
        is_transit: true,
    });
    let nizhny = graph.add_node(City {
        name: "Нижний Новгород",
        is_transit: true,
    });
    let petersburg = graph.add_node(City {
        name: "Санкт-Петербург",
        is_transit: false,
    });
    let kazan = graph.add_node(City {
        name: "Казань",
        is_transit: true,
    });
    let samara = graph.add_node(City {
        name: "Самара",
        is_transit: false,
    });

    graph.add_edge(
        moscow,
        nizhny,
        Road {
            price_rub: 12_000,
            lead_time_days: 2,
        },
    )?;
    graph.add_edge(
        moscow,
        petersburg,
        Road {
            price_rub: 8_000,
            lead_time_days: 1,
        },
    )?;
    graph.add_edge(
        nizhny,
        kazan,
        Road {
            price_rub: 9_000,
            lead_time_days: 2,
        },
    )?;
    graph.add_edge(
        kazan,
        samara,
        Road {
            price_rub: 7_000,
            lead_time_days: 1,
        },
    )?;
    graph.add_edge(
        petersburg,
        samara,
        Road {
            price_rub: 11_000,
            lead_time_days: 3,
        },
    )?;

    let strategy = ByCost::new(|edge: &Road| edge.price_rub);
    let constraint = TransitConstraint;

    let to_samara = shortest_path_with_constraint(&graph, moscow, samara, &strategy, &constraint)?;
    print_path("Дешевый маршрут Москва -> Самара", &graph, to_samara);

    Ok(())
}
