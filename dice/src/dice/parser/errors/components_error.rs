use super::ComponentError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComponentsError {
    #[error(transparent)]
    Component(#[from] ComponentError),
}
