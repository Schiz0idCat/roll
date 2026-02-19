use super::{RollResult, RollSetResult};

/// A polymorphic roll result.
///
/// `RollOutput` is produced when executing a [`RollExpr`](super::RollExpr).
/// It contains either a [`RollResult`] for a single roll,
/// or a [`RollSetResult`] for a roll set.
pub enum RollOutput {
    Single(RollResult),
    Set(RollSetResult),
}

impl std::fmt::Display for RollOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RollOutput::Single(r) => write!(f, "{r}"),
            RollOutput::Set(s) => write!(f, "{s}"),
        }
    }
}
