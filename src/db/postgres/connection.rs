//! This module provides functionality to connect to a PostgreSQL database using the `tokio_postgres` crate.

use crate::environment as env;
use tokio_postgres::{Config, NoTls};


/// Function to establish a connection to the PostgreSQL database.
pub async fn get_db_client() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let logname = env::get_logname();
    let db_password = env::get_db_password();
    let mut config: Config = tokio_postgres::Config::new();
    config
        .host("localhost")
        .user(&logname)
        .dbname("dominote")
        .password(&db_password)
        .port(5432); // Default PostgreSQL port

    let (client, connection) = config.connect(NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
