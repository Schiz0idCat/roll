use super::{RollResult, RollSetResult};

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
