use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use std::net::*;
use serde::{Serialize, Deserialize};

const FILENAME: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ip_address: String,
    port_number: String
}
impl Config {
    //constructor if needed
    fn new(address: String, port: String) -> Config {
        let new_config = Config {
            ip_address: address.clone(),
            port_number: port.clone()
        };
        new_config
    }

    //static method to create local server
    fn default() -> Config {
        let default_config = Config {
            ip_address: String::from("127.0.0.1"),
            port_number: String::from("8080")
        };
        default_config
    }
}

struct HandlerStruct {
    method: String,
    path: String,
    handler: fn(TcpStream)
}

/*
should create something like ServerStruct to contain multiple HandlerStruct
maybe B-trees
*/

//routing function, should be method of ServerStruct to bind method & path to handler function
fn routing(stream:TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut path = String::new();
    reader.read_line(&mut path).unwrap();
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
    for stream in socket.incoming() {
        //catch errors
        let stream = stream.unwrap();

        
    }
}
