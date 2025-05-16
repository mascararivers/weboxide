use thiserror::Error;

/// A specific type for error handling
pub type WeboxideResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
/// An error that could happen while making an API call
pub enum ApiError {
    #[error("Request error: {0}")]
    /// An error from [`reqwest`]
    RequestError(#[from] reqwest::Error),
}
