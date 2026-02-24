use super::super::Component;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ComponentError {
    #[error("Modifier must be a valid integer.")]
    InvalidModifier,

    #[error("Couldn't parse int.")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Component {0} is not recognized.")]
    InvalidComponent(String),

    #[error("These are mutually exclusive components: {0} - {1}")]
    ConflictingComponents(Component, Component),
}
