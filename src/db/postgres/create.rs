use crate::db::postgres::connection::get_db_client;
use tokio_postgres::Error;

pub async fn create_player(name: String) -> Result<u64, Error> {
    let client = get_db_client().await?;
    return client
        .execute("INSERT INTO players (name, score, wins, games_played) VALUES ($1, 0, 0, 0)", &[&name])
        .await;
}
