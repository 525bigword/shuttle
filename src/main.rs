use std::env;
use crate::command::Command;
mod device;
mod command;
mod socket;
mod event;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    println!("Searching for {}", arg1);
    let command=arg1.as_str().into();
    match command 
    {
        Command::Connect(ip)=>{
            if args.len()>2 {
                Command::Connect(args[2].as_str()).run();
                return;
            }
            command.run();
        },
        Command::Start(device)=>{
            if args.len()>2 {
                Command::Start(&Some(args[2].clone())).run();
            }
            command.run();
        },
        _ => command.run(),
    }
}

