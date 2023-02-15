use x11::xlib::{XDisplayWidth, XOpenDisplay, XDefaultScreen, XDisplayHeight};


pub fn get_system_metrics() -> (i32,i32){
    unsafe{
        let d = XOpenDisplay(std::ptr::null());
        let s = XDefaultScreen(d);
        let w = XDisplayWidth(d,s);
        let h = XDisplayHeight(d,s);
        (w,h)
    }
}


#[cfg(test)]
mod tests {
    use super::get_system_metrics;

    #[test]
    fn test_get_system_metrics(){
        get_system_metrics();
    }
}