use mouse_position::mouse_position::Mouse;

fn a() {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => println!("x: {}, y: {}", x, y),
        Mouse::Error => println!("Error getting mouse position"),
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn test_connect_device() {
        while true {
            let position = Mouse::get_mouse_position();
            match position {
                Mouse::Position { x, y } => println!("x: {}, y: {}", x, y),
                Mouse::Error => println!("Error getting mouse position"),
            }
            thread::sleep(time::Duration::from_millis(1000))
        }
    }

    fn test_dc(){
    }
}
