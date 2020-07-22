use std::fs;
use std::net::TcpListener;
use serde::{Serialize, Deserialize};

const FILENAME: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip_address: String,
    port_number: String
}

impl Config {
    fn default() -> Config {
        let default_config = Config {
            ip_address: String::from("127.0.0.1"),
            port_number: String::from("8080")
        };
        default_config
    }
}

fn main() {
    //read json config file to str
    let json_input = fs::read_to_string(FILENAME).expect("config file not found, aborting");
    //call serde_json::from_str(&input).unwrap() to deserialize
    let server_config: Config = serde_json::from_str(&json_input).unwrap();

    //create ip:port format
    let mut ip_port = server_config.ip_address.clone();
    ip_port.push_str(&server_config.port_number);

    //create socket
    let socket = TcpListener::bind(ip_port).unwrap();
    
    //read stream here
}
