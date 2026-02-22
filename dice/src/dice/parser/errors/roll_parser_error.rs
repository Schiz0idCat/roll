use super::ComponentError;
use crate::errors::DieError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RollParserError {
    #[error(transparent)]
    Component(#[from] ComponentError),

    #[error(transparent)]
    Die(#[from] DieError),

    #[error("Couldn't parse int.")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Cannot parse an empty die.")]
    EmptyDie,

    #[error("A die must have sides. Valid formats: [NdM, dM, M].")]
    NoSides,

    #[error("advantage or disadvantage rolls may use at most 2 dice")]
    InvalidAdvantageMultiplicity,
}
