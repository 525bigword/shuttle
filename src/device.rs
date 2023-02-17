use std::thread;

use bluest::{Adapter, Device, Uuid};
use log::info;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use tokio::runtime::{Runtime};
lazy_static! {
    pub static ref UUID_STR_VEC: RwLock<Vec<String>> = RwLock::new(Vec::new());
}

pub fn scan_device() {
    println!(".......................");
    let rt = Runtime::new().unwrap();

    let future = async {
        let adapter = Adapter::default()
            .await
            .ok_or("Bluetooth adapter not found")
            .unwrap();
        adapter.wait_available().await;

        info!("starting scan");
        let mut scan = adapter.scan(&[]).await.unwrap();
        info!("scan started");
        while let Some(discovered_device) = scan.next().await {
            info!(
                "name:{:?}----->services：{:?}",
                discovered_device
                    .device
                    .name_async()
                    .await
                    .unwrap_or("undefind".to_string()),
                // discovered_device.device.id(),
                discovered_device.adv_data.services
            );
        }
    };
    rt.block_on(future);
}

// 连接蓝牙
pub fn connect_device(device: &Device) {
    let rt = Runtime::new().unwrap();
    let future = Adapter::default();
    let adapter=rt.block_on(future).ok_or("Bluetooth adapter not found")
    .unwrap();
    adapter.connect_device(&device);
}

// 断开蓝牙
pub fn disconnect_device(device: &Device) {
    let rt = Runtime::new().unwrap();
    let future = Adapter::default();
    let adapter=rt.block_on(future).ok_or("Bluetooth adapter not found")
    .unwrap();
    adapter.disconnect_device(device);
}

// 锁定蓝牙设备
pub fn get_device(services: &[Uuid]) -> (Vec<String>, Device) {
    let rt = Runtime::new().unwrap();
    let future= async {
        let adapter = Adapter::default().await.unwrap();
        adapter.wait_available().await.unwrap();
        info!("looking for device");
        let device = adapter
            .discover_devices(services)
            .await
            .unwrap()
            .next()
            .await
            .ok_or("Failed to discover device")
            .unwrap()
            .unwrap();
        println!(
            "found device: {} ({:?})",
            device.name_async().await.as_deref().unwrap_or("(unknown)"),
            device.id()
        );
        let services = services.to_vec();
        let mut uuid_str=UUID_STR_VEC.write().await;
        for uuid in services {
            uuid_str.push(uuid.to_string());
        }
        (uuid_str.to_vec(),device)
    };
    let (a,b)=rt.block_on(future);
    println!("{:?}:{}",a,b);
    (a,b)
}

#[cfg(test)]
mod tests {
    use bluest::Uuid;

    use super::*;
    #[tokio::test]
    async fn test_scan() {
        scan_device()
    }

    fn get_device() -> Device {
        let services = &[
            Uuid::parse_str("0000110e-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000110b-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000111e-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000110c-0000-1000-8000-00805f9b34fb").unwrap(),
        ];
        let (uuid, device) = super::get_device(services);
        device
    }

    #[tokio::test]
    async fn test_connect_device() {
        let device = get_device();
        connect_device(&device)
    }

    #[tokio::test]
    async fn test_disconnect_device() {
        let device = get_device();
        disconnect_device(&device)
    }
}
