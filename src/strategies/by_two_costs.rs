use crate::strategies::PathStrategy;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TwoCostsState {
    pub primary_cost: u64,
    pub secondary_cost: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TwoCostsOrder {
    PrimaryThenSecondary,
    SecondaryThenPrimary,
}

/// Нейтральная стратегия с двумя критериями (лексикографическое сравнение).
pub struct ByTwoCosts<FP, FS> {
    primary_cost_of: FP,
    secondary_cost_of: FS,
    order: TwoCostsOrder,
}

impl<FP, FS> ByTwoCosts<FP, FS> {
    pub fn primary_then_secondary(primary_cost_of: FP, secondary_cost_of: FS) -> Self {
        Self {
            primary_cost_of,
            secondary_cost_of,
            order: TwoCostsOrder::PrimaryThenSecondary,
        }
    }

    pub fn secondary_then_primary(primary_cost_of: FP, secondary_cost_of: FS) -> Self {
        Self {
            primary_cost_of,
            secondary_cost_of,
            order: TwoCostsOrder::SecondaryThenPrimary,
        }
    }

    pub fn with_order(
        primary_cost_of: FP,
        secondary_cost_of: FS,
        order: TwoCostsOrder,
    ) -> Self {
        Self {
            primary_cost_of,
            secondary_cost_of,
            order,
        }
    }
}

impl<E, FP, FS> PathStrategy<E> for ByTwoCosts<FP, FS>
where
    FP: Fn(&E) -> u64,
    FS: Fn(&E) -> u64,
{
    type State = TwoCostsState;
    type Key = (u64, u64);

    fn start_state(&self) -> Self::State {
        TwoCostsState::default()
    }

    fn next_state(&self, prev: &Self::State, edge: &E) -> Self::State {
        TwoCostsState {
            primary_cost: prev.primary_cost + (self.primary_cost_of)(edge),
            secondary_cost: prev.secondary_cost + (self.secondary_cost_of)(edge),
        }
    }

    fn key(&self, state: &Self::State) -> Self::Key {
        match self.order {
            TwoCostsOrder::PrimaryThenSecondary => (state.primary_cost, state.secondary_cost),
            TwoCostsOrder::SecondaryThenPrimary => (state.secondary_cost, state.primary_cost),
        }
    }
}
