use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    NotFound(String),
    AlreadyExists(String),
    ConnectionError(String),
    TransactionError(String),
    Unknown(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::NotFound(entity) => write!(f, "Entity not found: {}", entity),
            DatabaseError::AlreadyExists(entity) => write!(f, "Entity already exists: {}", entity),
            DatabaseError::ConnectionError(msg) => write!(f, "Database connection error: {}", msg),
            DatabaseError::TransactionError(msg) => {
                write!(f, "Database transaction error: {}", msg)
            }
            DatabaseError::Unknown(msg) => write!(f, "Unknown database error: {}", msg),
        }
    }
}

impl From<DieselError> for DatabaseError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => DatabaseError::NotFound("Entity".to_string()),
            DieselError::DatabaseError(_, info) => {
                DatabaseError::Unknown(info.message().to_string())
            }
            DieselError::RollbackTransaction => {
                DatabaseError::TransactionError("Transaction rollback".to_string())
            }
            DieselError::AlreadyInTransaction => {
                DatabaseError::TransactionError("Already in transaction".to_string())
            }
            _ => DatabaseError::Unknown(format!("{:?}", error)),
        }
    }
}

impl std::error::Error for DatabaseError {}
