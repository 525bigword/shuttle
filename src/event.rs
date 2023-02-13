use std::{
    sync::{RwLock},
    thread::{self},
    time::{Duration},
};

use mouse_position::mouse_position::Mouse;

lazy_static! {
    pub static ref STATE: RwLock<u8> = RwLock::new(0);
}

fn get_mouse_coordinate() -> Result<(i32,i32),&'static str>{
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => Ok((x,y)),
        Mouse::Error => Err("获取鼠标坐标错误"),
    }
}

pub fn watch_mouse() {
    let mut state =STATE.write().unwrap();
    *state=0_u8;
    thread::spawn(||{
        while STATE.read().unwrap().eq(&0){
            let (x,y)=get_mouse_coordinate().unwrap();
            println!("{},{}",x,y);
            todo!("获取屏幕分辨率");
            todo!("判断当前鼠标是否达到屏幕边界");
            thread::sleep(Duration::from_millis(500));
        }
    });
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
