use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize};

use crate::db::postgres::create::create_player;

#[derive(Deserialize, Debug)]
pub struct RegisterBodyRequest {
    player_name: String,
}

#[post("/player/register")]
pub async fn register_player_route(item: web::Json<RegisterBodyRequest>) -> impl Responder {
    print!("Registering player: {}\n", item.player_name);
    match create_player(item.player_name.to_string()).await {
        Ok(_) => HttpResponse::Created().body("Player registered successfully"),
        Err(e) => {
            eprintln!("Error registering player: {}", e);
            HttpResponse::InternalServerError().body("Failed to register player")
        }
    }
}
