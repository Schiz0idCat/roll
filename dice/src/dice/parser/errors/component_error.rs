use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComponentError {
    #[error("Modifier must be a valid integer.")]
    InvalidModifier,

    #[error("Couldn't parse int.")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Component {0} is not recognized.")]
    InvalidComponent(String),
}
