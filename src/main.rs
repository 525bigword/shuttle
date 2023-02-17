use std::env::{self, set_var};
use log::debug;

use crate::{command::Command, socket::string_to_uuid_vec};
mod device;
mod command;
mod socket;
mod event;
#[path ="./os/mod.rs"]
mod os;
#[macro_use]
extern crate lazy_static;

 fn main() {
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
                debug!("{}",args[2]);
                Command::Connect(args[2].as_str()).run();
                return;
            }
            command.run();
        },
        Command::Start(_)=>{
            debug!("len:{:?}",args);
            debug!("len:{:?}",args[2]);
            debug!("len:{:?}",args[3]);
            let uuids=string_to_uuid_vec(args[2].split(",").map(|s|s.to_string()).collect());
            if args.len()>2 {
                Command::Start(&Some(uuids)).run();
                return;
            }
            command.run();
        },
        _ => command.run(),
    }
}

