#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),
}

impl crate::prelude::Error for Error {
    fn code(&self) -> tonic::Code {
        match self {
            Self::Sqlx(_) => tonic::Code::Internal,
        }
    }
    fn message(&self) -> String {
        match self {
            Self::Sqlx(_) => "Error while operating database".to_string(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
