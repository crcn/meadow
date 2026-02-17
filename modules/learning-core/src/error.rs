use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("graph error: {0}")]
    Graph(#[from] neo4rs::Error),

    #[error("graph deserialization error: {0}")]
    GraphDe(#[from] neo4rs::DeError),

    #[error("auth error: {0}")]
    Auth(String),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("ai error: {0}")]
    Ai(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    Internal(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
