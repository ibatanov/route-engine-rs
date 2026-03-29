pub mod by_cost;
pub mod by_two_costs;

/// Контракт стратегии оценки пути для алгоритмов поиска.
pub trait PathStrategy<E> {
    type State: Clone;
    type Key: Ord + Clone;

    /// Начальное состояние в стартовой вершине.
    fn start_state(&self) -> Self::State;

    /// Переход к новому состоянию при прохождении ребра.
    ///
    /// Для корректности Dijkstra значение `key(next_state)` не должно убывать
    /// относительно `key(prev)`.
    fn next_state(&self, prev: &Self::State, edge: &E) -> Self::State;

    /// Ключ сравнения для выбора лучшего пути.
    fn key(&self, state: &Self::State) -> Self::Key;
}
