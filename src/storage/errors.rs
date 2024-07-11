use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("object not found at: {path}")]
    ObjectNotFound { path: String },
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("upstream error: {message}")]
    Upstream { status_code: Option<u16>, message: String },
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::Upstream { status_code, .. } => StatusCode::from_u16(status_code.unwrap_or(500)).unwrap(),
            Error::ObjectNotFound { .. } => StatusCode::NOT_FOUND,
            Error::Reqwest(err) => err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
