use crate::strategies::PathStrategy;

/// Нейтральная стратегия минимизации пути по одному агрегированному cost.
pub struct ByCost<F> {
    cost_of: F,
}

impl<F> ByCost<F> {
    pub fn new(cost_of: F) -> Self {
        Self { cost_of }
    }
}

impl<E, F> PathStrategy<E> for ByCost<F>
where
    F: Fn(&E) -> u64,
{
    type State = u64;
    type Key = u64;

    fn start_state(&self) -> Self::State {
        0
    }

    fn next_state(&self, prev: &Self::State, edge: &E) -> Self::State {
        *prev + (self.cost_of)(edge)
    }

    fn key(&self, state: &Self::State) -> Self::Key {
        *state
    }
}
