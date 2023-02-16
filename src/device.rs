use std::thread;

use bluest::{Adapter, Device, Uuid};
use log::info;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;

lazy_static! {
    pub static ref UUID_STR_VEC: RwLock<Vec<String>> = RwLock::new(Vec::new());
}

pub fn scan_device() {
    let t = thread::spawn(|| async {
        let adapter = Adapter::default()
            .await
            .ok_or("Bluetooth adapter not found")
            .unwrap();
        adapter.wait_available().await;

        info!("starting scan");
        let mut scan = adapter.scan(&[]).await.unwrap();
        info!("scan started");
        while let Some(discovered_device) = scan.next().await {
            println!(
                "{:?}:{:?}:{:?}",
                discovered_device
                    .device
                    .name_async()
                    .await
                    .unwrap_or("undefind".to_string()),
                discovered_device.device.id(),
                discovered_device.adv_data
            );
        }
    });
    t.join();
}

// 连接蓝牙
pub fn connect_device(device: &Device) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = Adapter::default();
    let adapter=rt.block_on(future).ok_or("Bluetooth adapter not found")
    .unwrap();
    adapter.connect_device(&device);
}

// 断开蓝牙
pub fn disconnect_device(device: &Device) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = Adapter::default();
    let adapter=rt.block_on(future).ok_or("Bluetooth adapter not found")
    .unwrap();
    adapter.disconnect_device(device);
}

// 锁定蓝牙设备
pub fn get_device(services: &[Uuid]) -> (Vec<String>, Device) {
    let rt = tokio::runtime::Runtime::new().unwrap();
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
        let services = services.clone();
        for uuid in services {
            UUID_STR_VEC.write().await.push(uuid.to_string());
        }
        (UUID_STR_VEC.read().await.to_vec(),device)
    };
    rt.block_on(future)

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
