pub mod allow;

use crate::graph::NodeId;

/// Контекст текущего ребра в процессе поиска пути.
pub struct EdgeContext<'a, N, E> {
    pub source: NodeId,
    pub target: NodeId,
    pub from: NodeId,
    pub from_node: &'a N,
    pub to: NodeId,
    pub to_node: &'a N,
    pub edge: &'a E,
}

/// Контракт для бизнес-ограничений на прохождение ребра в поиске пути.
pub trait PathConstraint<N, E, St> {
    /// Возвращает `true`, если ребро разрешено для текущего шага маршрута.
    fn allow_edge(&self, ctx: &EdgeContext<'_, N, E>, prev_state: &St, next_state: &St) -> bool;
}
