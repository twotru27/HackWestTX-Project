#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("database access error: {0}")]
    MongoDBError(#[from] mongodb::error::Error),
}
