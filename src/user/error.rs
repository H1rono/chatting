#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
