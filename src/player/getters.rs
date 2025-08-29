use postgres::Row;

use crate::{
    db::postgres::commands::{show_table},
    player::{structs::Player},
};

pub async fn get_from_db_player_list() -> Result<Vec<Player>, tokio_postgres::Error> {
    // This function would typically fetch the player list from the database
    // For now, we return a placeholder string
    let list: Vec<Row> = show_table("players", vec!["*"]).await?;
    let mut players: Vec<Player> = Vec::new();
    for row in list.iter() {
        let player: Player = Player {
            name: row.get("name"),
            score: row.get("score"),        // Placeholder, as we are not fetching score here
            victories: row.get("wins"),    // Placeholder, as we are not fetching victories here
            games_played: row.get("games_played"), // Placeholder, as we are not fetching games played here
        };
        players.push(player);
    }
    return Ok(players);
}
