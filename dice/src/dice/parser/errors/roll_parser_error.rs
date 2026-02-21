use super::super::Component;
use super::{ComponentsError, ExtraError};
use crate::errors::DieError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RollParserError {
    #[error(transparent)]
    Component(#[from] ComponentsError),

    #[error(transparent)]
    Extra(#[from] ExtraError),

    #[error("Cannot parse an empty die.")]
    EmptyDie,

    #[error("A die must have sides.")]
    NoSides,

    #[error("Modifier must be a valid integer.")]
    InvalidModifier,

    #[error("Parse die error. Valid formats: NdM - dM - M.")]
    ParseDie,

    #[error("Couldn't parse int.")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Die(#[from] DieError),

    #[error("Component {0} is not recognized.")]
    InvalidComponent(String),

    #[error("These are mutually exclusive components: {0} - {1}")]
    ConflictingComponents(Component, Component),
}
