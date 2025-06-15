use tokio_postgres::{Config, Error, NoTls};

use crate::environment as env;

pub async fn create_player(name: String) -> Result<(), Error> {
    let logname =  env::get_logname();
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

    match client
        .execute("INSERT INTO players (name) VALUES ($1)", &[&name])
        .await {
        Ok(rows) => {
            println!("Inserted {} row(s)", rows);
        },
        Err(e) => {
            eprintln!("Error creating player: {}", e);
        }
    }
    Ok(())
}
