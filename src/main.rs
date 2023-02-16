use std::env::{self, set_var};
use bluest::Uuid;
use tokio::sync::RwLock;

use crate::command::Command;
mod device;
mod command;
mod socket;
mod event;
#[path ="./os/mod.rs"]
mod os;
#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    println!("Searching for {}", arg1);
    let command=arg1.as_str().into();

    match command 
    {
        Command::Connect(_)=>{
            if args.len()>2 {
                println!("{}",args[2]);
                Command::Connect(args[2].as_str()).run().await;
                return;
            }
            command.run();
        },
        Command::Start(_)=>{
            println!("len:{:?}",args);
            println!("len:{:?}",args[2]);
            println!("len:{:?}",args[3]);
            // let mut  services:Vec<Uuid>=Vec::new();
            // services.push(Uuid::parse_str("0000110e-0000-1000-8000-00805f9b34fb").unwrap());
            // services.push(Uuid::parse_str("0000110b-0000-1000-8000-00805f9b34fb").unwrap());
            // services.push(Uuid::parse_str("0000111e-0000-1000-8000-00805f9b34fb").unwrap());
            // services.push(Uuid::parse_str("0000110c-0000-1000-8000-00805f9b34fb").unwrap());
            // if args.len()>2 {
            //     Command::Start(&Some(services)).run();
            // }
            // command.run();
        },
        _ => command.run().await,
    }
}

