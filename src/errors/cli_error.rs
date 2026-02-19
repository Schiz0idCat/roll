use dice::errors::DieError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Die(#[from] DieError),

    #[error("Couldn't parse int")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Parse die error. Valid formats: NdM - dM - M")]
    ParseDie,

    #[error("advantage or disadvantage rolls may use at most 2 dice")]
    InvalidAdvantageMultiplicity,
}
