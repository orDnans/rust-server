use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use std::net::*;
use std::collections::BTreeMap;
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

// struct HandlerStruct {
//     method: String,
//     path: String,
//     handler: fn(TcpStream)
// }

/*
should create something like ServerStruct to contain multiple HandlerStruct
maybe B-trees
*/

fn placeholder(_stream:TcpStream) {}

struct TreeNode {    
    path: String,
    method: String,
    handler: fn(TcpStream),
    children: Vec<Option<Box<TreeNode>>>
}

impl TreeNode {
    fn new() -> TreeNode {
        let new_tree = TreeNode {
            path: String::from("/"),
            method: String::from(""),
            handler: placeholder,
            children: Vec::new()
        };
        new_tree
    }
    // fn insert(&mut self, method:String, path:String, handler:fn(TcpStream)) {
    //     if path.starts_with(&self.path) {
    //         let mut found : bool = false;
    //         for &child in self.children {
    //             match child {
    //                 Some(tree) => if path.starts_with(&tree.path) {
    //                     tree.insert(method, path, handler);
    //                     found = true;
    //                 },
    //                 None => continue,
    //             }
    //         }
    //         if found == false {
    //             //append new tree in this tree's children vector/list
    //             new_tree : TreeNode =  TreeNode.new()
    //             self.children.append()
    //         }
    //     }
    // }

    fn insert(&mut self, path:&str, method:&str, handler: fn(TcpStream)) {
        if path.eq(&self.path) && method.eq(&self.method){
            self.handler = handler;
            return;
        }
        if path.starts_with(&self.path) {
            let child_iter = self.children.iter_mut();
            let mut found : bool = false;
            for child in child_iter {
                match child {
                    Some(child_tree) => if path.starts_with(&child_tree.path) {
                        child_tree.insert(path, method, handler);
                        found = true;
                    },
                    None => continue,
                }
            }
            if found == false {
                let new_tree = TreeNode {
                    path: path.to_string(),
                    method: method.to_string(),
                    handler: handler,
                    children: Vec::new()
                };
                self.children.push(Some(Box::new(new_tree)));
            }
        }
    }

    //register a GET request on a path with a specific handler
    fn get(&mut self, get_path:&str, get_handler:fn(TcpStream)) {
        self.insert(get_path, "GET", get_handler);
    }

    fn post(&mut self, post_path:&str, post_handler:fn(TcpStream)) {
        self.insert(post_path, "POST", post_handler);
    }

    fn search(&self, path:&str, method:&str) -> Option<&fn(TcpStream)> {
        if path.eq(&self.path) {
            return Some(&self.handler);
        } else if path.starts_with(&self.path) {
            let child_iter = self.children.iter();
            for child in child_iter {
                match child {
                    Some(tree_child) => return tree_child.search(path, method),
                    None => continue,
                }
            }
        }
        None
    }

    fn routing(&self, stream:TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut path = String::new();
        reader.read_line(&mut path).unwrap();
        println!("{}", path);

        let mut handler : Option<&fn(TcpStream)> = None;

        if path.starts_with("GET") {
            let route = &path[4..path.len()-1];
            handler = self.search(route, "GET");
        } else if path.starts_with("POST") {
            let route = &path[5..path.len()-1];
            handler = self.search(route, "POST");
        }

        match handler {
            Some(func) => func(stream),
            None => println!("no handler"),
        }
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

    //create routing tree
    let router = TreeNode::new();
    
    //read stream here
    for stream in socket.incoming() {
        //catch errors
        let stream = stream.unwrap();

        
    }
}
