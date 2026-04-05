use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use route_engine_rs::algorithms::shortest_path;
use route_engine_rs::graph::{Graph, NodeId};
use route_engine_rs::strategies::by_cost::ByCost;
use std::hint::black_box;

fn build_graph(points: usize) -> (Graph<(), u64>, NodeId, NodeId) {
    let mut graph = Graph::new();
    let nodes: Vec<NodeId> = (0..points).map(|_| graph.add_node(())).collect();

    for i in 0..(points - 1) {
        let from = nodes[i];
        graph.add_edge(from, nodes[i + 1], 1).unwrap();

        if i + 2 < points {
            graph.add_edge(from, nodes[i + 2], 3).unwrap();
        }
        if i + 3 < points {
            graph.add_edge(from, nodes[i + 3], 5).unwrap();
        }
    }

    (graph, nodes[0], nodes[points - 1])
}

fn bench_shortest_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortest_path");
    let strategy = ByCost::new(|edge: &u64| *edge);

    for points in [500, 1000, 5000, 10000] {
        let (graph, from, to) = build_graph(points);
        group.throughput(Throughput::Elements(points as u64));

        group.bench_with_input(BenchmarkId::new("points", points), &points, |b, _| {
            b.iter(|| {
                let result = shortest_path(
                    black_box(&graph),
                    black_box(from),
                    black_box(to),
                    black_box(&strategy),
                )
                .expect("shortest_path must not fail")
                .expect("path must exist");

                black_box(result.state);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_shortest_path);
criterion_main!(benches);
