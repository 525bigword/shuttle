
extern crate ws;
use std::collections::HashMap;
use std::sync::{ RwLock};

use serde::{Deserialize, Serialize};
use ws::util::Token;
use ws::{listen, Message, Sender};
use ws::{connect as ws_connect, CloseCode};

use crate::device::lock_device;
use crate::event::{watch_mouse, STATE};

lazy_static! {
    static ref SENDER_LIST: RwLock<HashMap<Token,Sender>> = RwLock::new(HashMap::new());
}
pub fn start(device:String) {
    let device=lock_device(device);
    println!("已锁定设备:{}",device);
    
    let listen=listen("127.0.0.1:20426", |sender| {
        println!("out:{:?}",sender);
        let _ = &SENDER_LIST.write().unwrap().insert(sender.token(), sender.clone());
        move |msg:Message| {
            println!("received msg: {}", msg);
            // let so:Socket=msg.to_string().as_str().into();
            let mut message:Msg =serde_json::from_str(&msg.as_text().unwrap()).unwrap();
            match message.socket {
                Socket::LockDevice =>{
                    message.message=device.clone();
                    sender.send(Message::Text(message.into()))
                },
                Socket::Other => sender.send(msg),
            }
            
        }
    });
    match listen {
        Ok(_) => {
            watch_mouse();
            println!("开始监控鼠标");
        },
        Err(_) => {
            let mut state=STATE.write().unwrap();
            *state=1_u8;
        },
    }
}

pub fn connect(ip:&str) {
    let msg:String=Msg{
        socket:Socket::LockDevice,
        message: format!(""),
    }.into();
    watch_mouse();
    println!("开始监控鼠标");
    if let Err(error) = ws_connect(format!("ws://{}:20426",ip), |out| {
        // 将WebSocket打开时要发送的消息排队
        if out.send(msg.as_str()).is_err() {
            println!("Websocket无法初始消息排队")
        } else {
            println!("连接成功");
        }
        // 处理程序需要获取out的所有权，因此我们使用move
        move |msg:Message| {
            // 处理在此连接上接收的消息
            println!("Client 收到消息 '{}'. ", msg);
            let  message:Msg =serde_json::from_str(&msg.as_text().unwrap()).unwrap();
            let _=match message.socket {
                Socket::LockDevice => lock_device(message.message),
                Socket::Other => todo!(),
            };
            Ok(())
        }
    }) {
        // 通知用户故障
        println!("Failed to create WebSocket due to: {:?}", error);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Msg{
    pub socket:Socket,
    pub message:String
}

impl Msg {
   
}

impl From<Msg> for String {
    fn from(msg: Msg) -> String {
        serde_json::to_string(&msg).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Socket {
    LockDevice,
    Other,
}

impl Socket{
    fn message_to_socket(socket: &Message) -> Self {
        let LockDevice =Message::Text("LockDevice".to_string());
        let Other =Message::Text("Other".to_string());
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

    #[test]
    fn test_start() {
        start("1".to_string())
    }
    #[test]
    fn test_connect() {
        connect("127.0.0.1");
    }
}

