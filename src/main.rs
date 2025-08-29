use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use dotenvy::dotenv;
// use std::env;
mod db;
mod player;
mod environment;
mod routes;

use routes::player as player_routers; 
use environment::get_host_url as host;
use environment::get_server_port as port;
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server Running in http://{:}:{:}", host(), port());
    // let _ = create_player("John Doe".to_string()).await;
    // let _ = update_player_name("John Doe".to_string(), "Barry Allen".to_string()).await;
    // let _ = update_player_wins("John Doe".to_string(), 5, 7).await;
    HttpServer::new(|| {
        App::new()
        .service(player_routers::register::register_player_route)
        .service(player_routers::list::get_player_list)
            // .service(hello)
            // .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host(), port()))?
    .run()
    .await
}
