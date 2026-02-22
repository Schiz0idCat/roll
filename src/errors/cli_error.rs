use dice::parser::errors::RollParserError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    RollParserError(#[from] RollParserError),
}
