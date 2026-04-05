use crate::constraints::{EdgeContext, PathConstraint};

/// Ограничение по умолчанию: разрешает все ребра.
#[derive(Clone, Copy, Debug, Default)]
pub struct AllowAll;

impl<N, E, St> PathConstraint<N, E, St> for AllowAll {
    fn allow_edge(&self, _ctx: &EdgeContext<'_, N, E>, _prev_state: &St, _next_state: &St) -> bool {
        true
    }
}
