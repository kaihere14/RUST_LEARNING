use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Task not found with the id: {0}")]
    TaskNotFound(u32)
}