#![allow(unused_imports)]
use std::{
    collections::HashMap,
    fmt::format,
    hash::Hash,
    io::{ Read, Write },
    net::TcpListener,
    process::Command,
    sync::Mutex,
};
use serde::Serialize;
use serde::Deserialize;
use tokio::fs;

#[derive(Serialize, Deserialize)]
struct Database {
    data: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let mut db: HashMap<String, String> = load_db_from_file("data.json").await;
    println!("Logs from your program will appear here!");

    // Creating a TCP Listener
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("I'm listening!");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("{:?}", stream);
                println!("accepted new connection");
                loop {
                    stream.write(b">>").unwrap();
                    let mut buffer = [0u8; 128];
                    let byte_read = stream.read(&mut buffer).unwrap();
                    println!("Raw bytes: {:?}", &buffer[..byte_read]);
                    let data_received = String::from_utf8_lossy(&buffer[..byte_read]);
                    println!("data received: {:?}", data_received);
                    let res = command_executer(&data_received.trim(), &mut db);
                    let response = format!("{}\r\n", res);
                    stream.write(response.as_bytes()).unwrap();
                    save_to_json("data.json", &db).await;
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn load_db_from_file(path: &str) -> HashMap<String, String> {
    let file = fs::read_to_string(path).await.unwrap_or("{}".to_string());
    serde_json::from_str(&file).unwrap_or_default()
}

async fn save_to_json(file_name: &str, db: &HashMap<String, String>) {
    let json = serde_json::to_string_pretty(db).unwrap();
    fs::write(file_name, json).await.unwrap();
}

fn command_executer(command: &str, db: &mut HashMap<String, String>) -> String {
    let parts = command.split_whitespace().collect::<Vec<&str>>();
    // GET, SET, DEL
    match parts[0] {
        "GET" => {
            let key = parts[1];
            let value = db.get(key).unwrap_or(&"(nil)".to_string()).to_string();
            value
        }
        "SET" => {
            let key = parts[1];
            let value = match parts.get(2).filter(|v| !v.trim().is_empty()) {
                Some(v) => v,
                None => return "NEED VALUE".to_string(),
            };
            
            db.insert(key.to_string(), value.to_string());
            "OK".to_string()
        }
        "DEL" => {
            let key = parts[1];
            db.remove(key);
            "OK".to_string()
        }
        _ => {
            println!("Invalid command");
            "Invalid Command".to_string()
        }
    }
}
