#![allow(clippy::let_unit_value)]
use bluest::{Adapter, AdvertisingDevice, Device, Uuid};
use tokio::sync::RwLock;
use std::error::Error;
use std::mem::MaybeUninit;
use std::sync::Once;
use std::thread;
use std::time::Duration;
use tokio_stream::StreamExt;

lazy_static! {
    pub static ref UUID_STR_VEC: RwLock<Vec<String>> = RwLock::new(Vec::new());
}


pub async fn scan_device() {
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")
        .unwrap();
    adapter.wait_available().await;

    println!("starting scan");
    let mut scan = adapter.scan(&[]).await.unwrap();
    println!("scan started");
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
}

// 连接蓝牙
pub async fn connect_device(device:&Device){
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")
        .unwrap();
    println!(
        "{:?}:{:?}",
        device.name_async().await, device.id()
    );
    let _=adapter.connect_device(device).await;
    println!("connected!");
}

// 断开蓝牙
pub async fn disconnect_device(device:&Device) {
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")
        .unwrap();
    println!(
        "{:?}:{:?}",
        device.name_async().await, device.id()
    );
    adapter.disconnect_device(device);
}

// 锁定蓝牙设备
pub async fn lock_device(services: &[Uuid]) -> (Vec<String>,Device) {
    let device_id = {
        let adapter = Adapter::default().await.unwrap();
        adapter.wait_available().await.unwrap();
        println!("looking for device");
        let device = adapter
            .discover_devices(services)
            .await.unwrap()
            .next()
            .await
            .ok_or("Failed to discover device").unwrap().unwrap();
        println!(
            "found device: {} ({:?})",
            device.name_async().await.as_deref().unwrap_or("(unknown)"),
            device.id()
        );

        device.id()
    };
    println!("Time passes...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    {
        let adapter = Adapter::default().await.unwrap();
        adapter.wait_available().await;

        println!("re-opening previously found device");
        let device = adapter.open_device(&device_id).await.unwrap();
        println!(
            "re-opened device: {} ({:?})",
            device.name_async().await.as_deref().unwrap_or("(unknown)"),
            device.id()
        );
        let services=services.clone();
        for uuid in services {
            UUID_STR_VEC.write().await.push(uuid.to_string());
        }
        (UUID_STR_VEC.read().await.to_vec(),device)
    }
}

#[cfg(test)]
mod tests {
    use bluest::{btuuid, Adapter, Uuid};
    use serde::__private::de;

    use super::*;
    #[tokio::test]
    async fn test_scan() {
        let a = scan_device().await;
    }

     fn get_device() -> Device{
        let services= &[
            Uuid::parse_str("0000110e-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000110b-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000111e-0000-1000-8000-00805f9b34fb").unwrap(),
            Uuid::parse_str("0000110c-0000-1000-8000-00805f9b34fb").unwrap(),
        ];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = lock_device(services);
        let (services,device)=rt.block_on(future);
        device
    }


    #[tokio::test]
    async fn test_connect_device() {
        let device=get_device();
        connect_device(&device).await;
    }

    #[tokio::test]
    async fn test_disconnect_device() {
        let device=get_device();
        disconnect_device(&device);
    }

}
