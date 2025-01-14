mod constants;
mod core;
mod game;
mod routes;
mod utils;

use anyhow::Result;
use constants::{ascii::TITLE, config::CONFIG_JSON_PATH};
use routes::routes;
use utils::{json::read_json, server::Server};

#[tokio::main]
async fn main() -> Result<()> {
    // TITLE
    println!(r#"{}"#, TITLE);
    println!("IN CASE YOU PAID MONEY FOR THIS, YOU'VE BEEN SCAMMED.");
    println!("       THIS IS A FREE AND OPEN SOURCE PROJECT.       ");

    // SERVER
    let (server_address, server_port) = get_server_address();
    let server = Server::new(server_address, server_port);
    Ok(server.serve(routes()).await?)
}

fn get_server_address() -> (String, u64) {
    let config = read_json(CONFIG_JSON_PATH);
    let server_config = &config["server"];
    let host = server_config["host"].as_str().unwrap();
    let port = server_config["port"].as_u64().unwrap();
    (host.to_string(), port)
}
