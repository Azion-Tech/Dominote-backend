use tokio_postgres::{error::SqlState, Error};

pub fn handle_db_errors(error: Error) -> String {
    if error.code() == Some(&SqlState::UNIQUE_VIOLATION) {
        println!("Unique constraint violated: {}", error);
        return "Player already exists\n".to_string();
    }
    println!("Database error: {}", error);
    return error.to_string();
}
