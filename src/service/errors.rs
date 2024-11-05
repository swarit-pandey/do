use crate::db::errors::DatabaseError;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    InvalidInput(String),
    OperationFailed(String),
    Database(DatabaseError),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ServiceError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            ServiceError::Database(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceError::Database(err) => Some(err),
            _ => None,
        }
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(err: DatabaseError) -> Self {
        ServiceError::Database(err)
    }
}
