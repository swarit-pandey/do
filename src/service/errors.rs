use crate::db::errors::DatabaseError;

#[derive(Debug)]
pub enum ServiceError {
    InvalidInput(String),
    OperationFailed(String),
    Database(DatabaseError),
}

impl From<DatabaseError> for ServiceError {
    fn from(err: DatabaseError) -> Self {
        ServiceError::Database(err)
    }
}
