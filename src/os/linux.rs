

pub fn get_system_metrics() -> (i32,i32){
    let mut x;
    let mut y;
    unsafe{
        const SPI_IOC_MAGIC: u8 = b'k'; // Defined in linux/spi/spidev.h
        const SPI_IOC_TYPE_MODE: u8 = 1;
        ioctl!(spi_read_mode, SPI_IOC_MAGIC, SPI_IOC_TYPE_MODE, u8);
    }
    (x,y)
}


#[cfg(test)]
mod tests {
    use super::get_system_metrics;

    #[test]
    fn test_get_system_metrics(){
        get_system_metrics();
    }
}