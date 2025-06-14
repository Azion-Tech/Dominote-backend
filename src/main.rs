

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use dotenvy::dotenv;
// use std::env;
mod environment;

use environment::get_host_url as host;
use environment::get_server_port as port;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server Running in http://{:}:{:}", host(), port());
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host(), port()))?
    .run()
    .await

    
}