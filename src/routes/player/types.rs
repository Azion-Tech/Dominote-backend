use serde::Serialize;

use crate::player::structs::Player;

#[derive(Serialize)]
pub struct PlayerListResponse {
    pub players: Vec<Player>,
}
