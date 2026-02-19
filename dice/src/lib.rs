mod dice;
pub mod errors;

pub use dice::{Die, Roll, RollExpr, RollResult, RollSet, RollSetResult, RollType, Rollable};
pub use errors::DieError;
