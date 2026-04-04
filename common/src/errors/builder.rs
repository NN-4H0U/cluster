use thiserror::Error;
#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Missing required field: '{field}'")]
    MissingField {
        field: &'static str,
    },

    #[error("Invalid value for field '{field}': '{value}' (expected: {expected})")]
    InvalidValue {
        field: &'static str,
        value: String,
        expected: String,
    },

    #[error("Field '{field}' is Invalid: {message}")]
    InvalidField {
        field: &'static str,
        message: String,
    }
}

pub type BuilderResult<T> = Result<T, BuilderError>;
