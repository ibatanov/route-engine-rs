use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::constraints::allow::AllowAll;
use crate::constraints::{EdgeContext, PathConstraint};
use crate::errors::DijkstraError;
use crate::graph::{Graph, NodeId};
use crate::strategies::PathStrategy;

#[derive(Debug)]
pub struct PathResult<St> {
    pub nodes: Vec<NodeId>,
    pub state: St,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct QueueEntry<K> {
    key: K,
    node: NodeId,
}

impl<K: Ord> Ord for QueueEntry<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .key
            .cmp(&self.key)
            .then_with(|| self.node.index().cmp(&other.node.index()))
    }
}

impl<K: Ord> PartialOrd for QueueEntry<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<N, E, S>(
    graph: &Graph<N, E>,
    from: NodeId,
    to: NodeId,
    strategy: &S,
) -> Result<Option<PathResult<S::State>>, DijkstraError>
where
    S: PathStrategy<E>,
{
    let constraint = AllowAll;
    shortest_path_with_constraint(graph, from, to, strategy, &constraint)
}

pub fn shortest_path_with_constraint<N, E, S, C>(
    graph: &Graph<N, E>,
    from: NodeId,
    to: NodeId,
    strategy: &S,
    constraint: &C,
) -> Result<Option<PathResult<S::State>>, DijkstraError>
where
    S: PathStrategy<E>,
    C: PathConstraint<N, E, S::State>,
{
    let node_count = graph.node_count();
    if from.index() >= node_count {
        return Err(DijkstraError::SourceNodeOutOfBounds {
            index: from.index(),
            node_count,
        });
    }
    if to.index() >= node_count {
        return Err(DijkstraError::TargetNodeOutOfBounds {
            index: to.index(),
            node_count,
        });
    }

    let mut best: Vec<Option<(S::Key, S::State)>> = vec![None; node_count];
    let mut prev: Vec<Option<NodeId>> = vec![None; node_count];
    let mut heap: BinaryHeap<QueueEntry<S::Key>> = BinaryHeap::with_capacity(node_count);

    let start_state = strategy.start_state();
    let start_key = strategy.key(&start_state);
    best[from.index()] = Some((start_key.clone(), start_state));
    heap.push(QueueEntry {
        key: start_key,
        node: from,
    });

    while let Some(entry) = heap.pop() {
        let entry_idx = entry.node.index();
        let Some((best_key, best_state_ref)) = best[entry_idx].as_ref() else {
            continue;
        };

        if entry.key > *best_key {
            continue;
        }

        if entry.node == to {
            break;
        }

        let Some(from_node) = graph.node(entry.node) else {
            continue;
        };

        let current_state = best_state_ref.clone();
        let Some(neighbors) = graph.neighbors(entry.node) else {
            continue;
        };
        for (next_node, edge) in neighbors {
            let Some(to_node) = graph.node(*next_node) else {
                continue;
            };
            let edge_ctx = EdgeContext {
                source: from,
                target: to,
                from: entry.node,
                from_node,
                to: *next_node,
                to_node,
                edge,
            };

            let next_state = strategy.next_state(&current_state, edge);

            if !constraint.allow_edge(&edge_ctx, &current_state, &next_state) {
                continue;
            }
            let next_key = strategy.key(&next_state);

            let next_idx = (*next_node).index();

            let should_relax = match best[next_idx].as_ref() {
                None => true,
                Some((known_key, _)) => next_key < *known_key,
            };

            if should_relax {
                best[next_idx] = Some((next_key.clone(), next_state));
                prev[next_idx] = Some(entry.node);
                heap.push(QueueEntry {
                    key: next_key,
                    node: *next_node,
                });
            }
        }
    }

    let final_state = best
        .get(to.index())
        .and_then(|entry| entry.as_ref())
        .map(|(_, state)| state.clone());

    let Some(final_state) = final_state else {
        return Ok(None);
    };

    let mut path = Vec::new();
    let mut current = to;
    path.push(current);
    while current != from {
        let Some(next) = prev.get(current.index()).and_then(|entry| *entry) else {
            return Ok(None);
        };
        current = next;
        path.push(current);
    }
    path.reverse();

    Ok(Some(PathResult {
        nodes: path,
        state: final_state,
    }))
}
