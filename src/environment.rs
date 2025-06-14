use std::env;
use dotenvy::dotenv;

pub fn get_host_url() -> String {
        dotenv().ok();
        match env::var("HOST_URL") {
            Ok(url) => url,
            Err(_) => panic!("No HOST_URL set"),
        }
}

pub fn get_server_port() -> u16 {
    dotenv().ok();
    match env::var("SERVER_PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => panic!("No SERVER_PORT set"),
    }
}
