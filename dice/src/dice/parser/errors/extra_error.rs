use super::super::Component;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtraError {
    #[error("These are mutually exclusive components: {0} - {1}")]
    ConflictingComponents(Component, Component),
}
