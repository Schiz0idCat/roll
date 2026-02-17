use thiserror::Error;

#[derive(Debug, Error)]
pub enum DieError {
    #[error("die not recognized")]
    DieNotRecognized,
}
