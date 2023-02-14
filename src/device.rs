#![allow(clippy::let_unit_value)]
use bluest::{Adapter, Device, AdvertisingDevice};
use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_session::BluetoothSession;
use std::mem::MaybeUninit;
use std::sync::Once;
use std::thread;
use std::time::Duration;

pub fn scan_device() {
    let bt_session = &BluetoothSession::create_session(None).unwrap();
    let adapter: BluetoothAdapter = BluetoothAdapter::init(bt_session).unwrap();
    let adapter_id = adapter.get_id();
    // 创建蓝牙搜索的Session
    let discover_session =
        BluetoothDiscoverySession::create_session(&bt_session, adapter_id).unwrap();
    // 开始扫描设备
    discover_session.start_discovery().unwrap();
    print!("扫描中....");
    // 等待几秒
    thread::sleep(Duration::from_secs(10));
    // 获取设备列便
    let device_list = adapter.get_device_list().unwrap();
    // 结束扫描
    // discover_session.stop_discovery().unwrap();

    for device_path in device_list {
        let device = BluetoothDevice::new(bt_session, device_path.to_string());
        println!(
            "Device: {:?} Name: {:?}, RSSI: {:?}",
            device_path,
            device.get_name().ok(),
            device.get_rssi().ok()
        );
    }
}

// 连接蓝牙
fn connect_device<'a>(bt_session: &'a BluetoothSession, address: &'a str) -> BluetoothDevice<'a> {
    let device = BluetoothDevice::new(
        bt_session,
        String::from(address), // mmc
    );

    if let Err(e) = device.connect(10000) {
        println!("Failed to connect {:?}: {:?}", device.get_id(), e);
    } else {
        println!("Connected!");
    }
    device
}

// 断开蓝牙
fn disconnect_device(device: &BluetoothDevice) {
    if let Err(e) = device.disconnect() {
        println!("Failed to connect {:?}: {:?}", device.get_id(), e);
    } else {
        println!("Disconnected!");
    }
}

// 获取蓝牙
pub fn lock_device(device: String) -> &'static String {
    static mut DEVICE: MaybeUninit<String> = MaybeUninit::uninit();
    static LOCK: Once = Once::new();
    LOCK.call_once(|| unsafe {
        DEVICE.as_mut_ptr().write(device);
    });
    unsafe { &*DEVICE.as_ptr() }
}


use tokio_stream::StreamExt;
pub async  fn a()  -> Result<(),&'static str>{
    let adapter = Adapter::default()
            .await
            .ok_or("Bluetooth adapter not found").unwrap();
        adapter.wait_available().await;

        println!("starting scan");
        let mut scan = adapter.scan(&[]).await.unwrap();
        println!("scan started");
        Ok(while let Some(discovered_device) = scan.next().await {
            println!("{:?}",discovered_device);
            // println!(
            //     "{}{}: {:?}",
            //     discovered_device
            //         .device
            //         .name()
            //         .as_deref()
            //         .unwrap_or("(unknown)"),
            //     discovered_device
            //         .rssi
            //         .map(|x| format!(" ({}dBm)", x))
            //         .unwrap_or_default(),
            //     discovered_device.adv_data.services
            // );
        })
}


#[cfg(test)]
mod tests {
    use bluest::Adapter;

    use super::*;

    #[test]
    fn test_connect_device() {
        let bt_session = &BluetoothSession::create_session(None).unwrap();
        connect_device(bt_session, "/org/bluez/hci0/dev_41_42_8C_A9_68_60");
    }
    #[test]
    fn test_disconnect_device() {
        let bt_session = &BluetoothSession::create_session(None).unwrap();
        let device: BluetoothDevice =
            connect_device(bt_session, "/org/bluez/hci0/dev_41_42_8C_A9_68_60");
        disconnect_device(&device);
    }

    #[tokio::test]
    async fn  test_disconnect_device1(){
        let a =a().await;
        println!("a:{:?}",a)
    }
}
