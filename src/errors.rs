#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Database connection error: ")]
    DbConnection(#[from] r2d2::Error),
    #[error("Database execution error: ")]
    DbExecution(#[from] diesel::result::Error),
    #[error("IO Error: ")]
    IO(#[from] std::io::Error)
}
