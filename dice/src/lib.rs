mod dice;
pub mod errors;

pub use dice::parser;
pub use dice::{
    Die, Roll, RollExpr, RollOutput, RollResult, RollSet, RollSetResult, RollType, Rollable,
};
pub use errors::DieError;
