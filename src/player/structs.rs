// This file is part of Dominote.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub victories: i32,
    pub games_played: i32,
}
