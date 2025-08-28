use actix_web::{get, HttpResponse, Responder};

use crate::{player::getters::get_from_db_player_list, routes::player::types::PlayerListResponse};

#[get("/player/list")]
async fn get_player_list() -> impl Responder {
    let list = get_from_db_player_list().await;
    match list {
        Ok(players) => {
            let data : PlayerListResponse = PlayerListResponse { players };
            HttpResponse::Ok().json(data)
        }
        Err(e) => {
            eprintln!("Error fetching player list: {}", e);
            return HttpResponse::InternalServerError().body("Failed to fetch player list");
        }
        
    }
}
