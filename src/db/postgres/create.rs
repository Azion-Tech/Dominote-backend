
use crate::db::postgres::connection::get_db_client;
use tokio_postgres::Error;

pub async fn create_player(name: String) -> Result<(), Error> {
    let client = get_db_client().await?;
    match client
        .execute("INSERT INTO players (name, score, wins, games_played) VALUES ($1, 0, 0, 0)", &[&name])
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
