
use bluest::Uuid;

use crate::{device::scan_device, socket::{start, connect}};

pub enum Command<'a> {
    Scan,
    Start(&'a Option<Vec<Uuid>>),
    Connect(&'a str),
}

impl Command<'_> {
    pub fn run(&self){
        match self {
            Command::Scan => scan_device(),
            Command::Start(device) =>{
                // print!("device:{}",device);
                match device {
                    Some(i) => start(i),
                    None => panic!("缺少设备号！"),
                };
                
            },
            Command::Connect(ip) =>connect(ip),
        }
    } 
}

impl From<&str> for Command<'_> {
    fn from(command: &str) -> Self {
        match command {
            "scan"=>Command::Scan,
            "start"=>{Command::Start(&None)},
            "con"=>Command::Connect("localhost"),
            _=> todo!()
        }
    }
}

