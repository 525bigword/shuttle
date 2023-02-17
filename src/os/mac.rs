pub fn get_system_metrics() -> (i32,i32){
    let mode = core_graphics::display::CGDisplay::main()
        .display_mode()
        .unwrap();

    println!(
        "{} x {}; {} x {}",
        mode.width(),
        mode.height(),
        mode.pixel_width(),
        mode.pixel_height()
    );
    (mode.pixel_width(),mode.pixel_height())
}


#[cfg(test)]
mod tests {
    use super::get_system_metrics;

    #[test]
    fn test_get_system_metrics(){
        get_system_metrics();
    }
}