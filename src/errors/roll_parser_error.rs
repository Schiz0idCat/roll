use super::DieError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RollParserError {
    #[error("Cannot parse an empty die.")]
    EmptyDie,

    #[error("A die must have sides")]
    NoSides,

    #[error("Parse die error. Valid formats: NdM - dM - M")]
    ParseDie,

    #[error("Couldn't parse int")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Die(#[from] DieError),
}
