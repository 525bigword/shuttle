extern crate ws;
use std::collections::HashMap;
use std::sync::RwLock;

use bluest::Uuid;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use ws::connect as ws_connect;
use ws::{listen, CloseCode, Message, Sender};

use crate::device::{get_device, UUID_STR_VEC};
use crate::event::watch_mouse;

lazy_static! {
    pub static ref ENCLOSURE_SOCKET: RwLock<HashMap<String, Sender>> = RwLock::new(HashMap::new());
}
pub fn start(services: &[Uuid]) {
    let (_uuid_str_vec, device) = get_device(services);
    info!("已锁定设备:{}", device);
    watch_mouse();
    let listen = listen("127.0.0.1:20426", |sender| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        debug!("out:{:?}", sender);
        let future = UUID_STR_VEC.read();
        let uuid = rt.block_on(future).to_vec();
        let _ = &ENCLOSURE_SOCKET
            .write()
            .unwrap()
            .insert(Direction::Left.into(), sender.clone());
        move |msg: Message| {
            debug!("received msg: {}", msg);
            let message_rusult: Result<Msg, Error> = serde_json::from_str(&msg.as_text().unwrap());
            match message_rusult {
                Ok(mut message) => match message.socket {
                    Socket::LockDevice => {
                        message.uuid_str_vec = uuid.clone();
                        sender.send(Message::Text(message.into()))
                    }
                    Socket::Other => sender.send(msg),
                    _ => sender.send(msg),
                },
                Err(_) => sender.close(CloseCode::Policy),
            }
        }
    });
    match listen {
        Ok(_) => {println!("1")},
        Err(_) => {println!("2")},
    }
}

pub fn connect(ip: &str) {
    let msg: String = Msg {
        socket: Socket::LockDevice,
        message: format!(""),
        uuid_str_vec: vec![],
    }
    .into();
    debug!("connect:{ip}");
    watch_mouse();
    info!("开始追踪鼠标");
    if let Err(error) = ws_connect(format!("ws://{}:20426", ip), |out| {
        // 将WebSocket打开时要发送的消息排队
        if out.send(msg.as_str()).is_err() {
            info!("Websocket无法初始消息排队")
        } else {
            info!("连接成功");
        }
        // 处理程序需要获取out的所有权，因此我们使用move
        move |msg: Message| {
            // 处理在此连接上接收的消息
            println!("Client 收到消息 '{}'. ", msg);
            let message: Msg = serde_json::from_str(&msg.as_text().unwrap()).unwrap();
            let _ = match message.socket {
                Socket::LockDevice => {
                    let str_list: Vec<String> = message.uuid_str_vec;
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let future = UUID_STR_VEC.write();
                    let mut uuid = rt.block_on(future).to_vec();
                    uuid = str_list;
                    //保存附件设备
                    let _ = &ENCLOSURE_SOCKET
                        .write()
                        .unwrap()
                        .insert(Direction::Right.into(), out.clone());
                }
                Socket::Other => todo!(),
                _ => {
                    todo!()
                }
            };
            Ok(())
        }
    }) {
        // 通知用户故障
        println!("Failed to create WebSocket due to: {:?}", error);
    }
}

pub fn string_to_uuid_vec(str_list: Vec<String>) -> Vec<Uuid> {
    let mut uuid: Vec<Uuid> = Vec::new();
    for str in str_list {
        uuid.push(Uuid::parse_str(str.as_str()).unwrap());
    }
    uuid
    // let services = &[
    //         Uuid::parse_str("0000110e-0000-1000-8000-00805f9b34fb").unwrap(),
    //         Uuid::parse_str("0000110b-0000-1000-8000-00805f9b34fb").unwrap(),
    //         Uuid::parse_str("0000111e-0000-1000-8000-00805f9b34fb").unwrap(),
    //         Uuid::parse_str("0000110c-0000-1000-8000-00805f9b34fb").unwrap(),
    //     ];
}

pub enum Direction {
    Top,
    Down,
    Left,
    Right,
}
impl From<Direction> for String {
    fn from(d: Direction) -> String {
        match d {
            Direction::Top => "top".to_string(),
            Direction::Down => "down".to_string(),
            Direction::Left => "left".to_string(),
            Direction::Right => "right".to_string(),
        }
    }
}

impl From<Direction> for &str {
    fn from(d: Direction) -> &'static str {
        match d {
            Direction::Top => "top",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub socket: Socket,
    pub message: String,
    pub uuid_str_vec: Vec<String>,
}

impl Msg {}

impl From<Msg> for String {
    fn from(msg: Msg) -> String {
        serde_json::to_string(&msg).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Socket {
    LockDevice,
    Connect,
    Other,
}

impl Socket {
    fn message_to_socket(socket: &Message) -> Self {
        let LockDevice = Message::Text("LockDevice".to_string());
        let Other = Message::Text("Other".to_string());
        match socket {
            // "LockDevice"=>Socket::LockDevice,
            // _=> todo!(),
            LockDevice => Socket::LockDevice,
            Other => Socket::Other,
        }
    }
}

// impl From<Message> for Socket {
//     fn from(socket: Message) -> Self {
//         let LockDevice =Message::Text("LockDevice".to_string());
//         let Other =Message::Text("Other".to_string());
//         match socket {
//             LockDevice => Socket::LockDevice,
//             Other => Socket::Other,
//         }
//     }
// }

// impl From<Socket> for String {
//     fn from(socket: Socket) -> String {
//         match socket {
//             Socket::LockDevice => "LockDevice".to_string(),
//             Socket::Other => "Other".to_string(),
//         }
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_start() {
        let services = &[
            Uuid::parse_str("0000110e-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000110b-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000111e-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000110c-0000-1000-8000-00805f9b34fb").unwrap(),
        ];
        start(services)
    }
    #[test]
    fn test_connect() {
        connect("127.0.0.1");
    }
}
