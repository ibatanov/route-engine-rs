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
pub trait PathConstraint<N, E> {
    /// Возвращает `true`, если ребро разрешено для текущего шага маршрута.
    fn allow_edge(&self, ctx: &EdgeContext<'_, N, E>) -> bool;
}

/// Ограничение по умолчанию: разрешает все ребра.
#[derive(Clone, Copy, Debug, Default)]
pub struct AllowAll;

impl<N, E> PathConstraint<N, E> for AllowAll {
    fn allow_edge(&self, _ctx: &EdgeContext<'_, N, E>) -> bool {
        true
    }
}
