//! This module handles environment variables for the application.

use dotenvy::dotenv;
use std::env;

/// Retrieves the host URL from the environment variables.
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

pub fn get_db_password() -> String {
    dotenv().ok();
    match env::var("DB_PASSWORD") {
        Ok(password) => password,
        Err(_) => panic!("No DB_PASSWORD set"),
    }
}

pub fn get_logname() -> String {
    dotenv().ok();
    match env::var("LOGNAME") {
        Ok(name) => name,
        Err(_) => panic!("No LOGNAME set"),
    }
}
