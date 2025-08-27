use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize};

use crate::db::postgres::{create::create_player, errors::create::{handle_db_errors}};

#[derive(Deserialize, Debug)]
pub struct RegisterBodyRequest {
    name: String,
}

#[post("/player/register")]
pub async fn register_player_route(item: web::Json<RegisterBodyRequest>) -> impl Responder {
    print!("Registering player: {}\n", item.name);
    match create_player(item.name.to_string()).await {
        Ok(_) => HttpResponse::Created().body("Player registered successfully"),
        Err(e) => {
            let error_message = handle_db_errors(e);
            HttpResponse::InternalServerError().body(error_message)
            // eprintln!("Error registering player: {}", e);
            // HttpResponse::InternalServerError().body("Failed to register player")
        }
    }
}
