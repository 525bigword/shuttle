use std::{
    sync::RwLock,
    thread::{self},
    time::Duration, collections::HashMap,
};

use log::{info, debug};
use mouse_position::mouse_position::Mouse;
use ws::{Message, Sender};

use crate::{
    device::{disconnect_device, get_device, UUID_STR_VEC},
    os::get_system_metrics,
    socket::{ENCLOSURE_SOCKET, string_to_uuid_vec, Msg, Socket, Direction},
};

lazy_static! {
    pub static ref STATE: RwLock<u8> = RwLock::new(0);
}

fn get_mouse_coordinate() -> Result<(i32, i32), &'static str> {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => Ok((x, y)),
        Mouse::Error => Err("获取鼠标坐标错误"),
    }
}

pub  fn watch_mouse() {
    let mut state = STATE.write().unwrap();
    *state = 0_u8;
    println!("111111111111");
    thread::spawn(|| {
        while STATE.read().unwrap().eq(&0) {
            let (x, y) = get_mouse_coordinate().unwrap();
            debug!("{},{}", x, y);
            let (width, height) = get_system_metrics();
            let sender_map = ENCLOSURE_SOCKET.read().unwrap();
            //左
            if x == 0 {
                changing(&sender_map,Direction::Left);
                debug!("达到左边界");
            } else if x == width-1 {
                //右
                changing(&sender_map,Direction::Right);
                debug!("达到右边界");
            } else if y == height-1 {
                //下
                changing(&sender_map,Direction::Down);
                debug!("达到下边界");
            } else if y == 0 {
                //上
                changing(&sender_map,Direction::Top);
                debug!("达到上边界");
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
}
///该函数用于判断鼠标是否需要切换设备，如果需要则切换
/// This function is used to determine whether the mouse needs to switch devices. If so, switch
fn changing(sender_map:&HashMap<String,Sender>,key:Direction) {
    let serder=sender_map.get(key.into());
    if serder.is_some() {
        //如果左边有设备则断开蓝牙发送给该设备连接蓝牙的通知
        //获取uuid
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = UUID_STR_VEC.read();
        let uuid_str = rt.block_on(future).to_vec();
        let uuid_list = string_to_uuid_vec(uuid_str);
        //获取蓝牙设备
        let (str, device) = get_device(&uuid_list);
        //断开蓝牙
        disconnect_device(&device);
        //发起通知
        let message: Msg = Msg {
            socket: Socket::Connect,
            message: format!(""),
            uuid_str_vec: str,
        };
        let _=serder
            .unwrap()
            .send(Message::Text(message.into()));
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn test_connect_device() {
        loop {
            let position = Mouse::get_mouse_position();
            match position {
                Mouse::Position { x, y } => println!("x: {}, y: {}", x, y),
                Mouse::Error => println!("Error getting mouse position"),
            }
            thread::sleep(time::Duration::from_millis(1000))
        }
    }
}
