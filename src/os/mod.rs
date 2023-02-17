#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod win;
#[cfg(mac)]
mod mac;


///不同平台同功能api
/// Same function api on different platforms
#[cfg(unix)]
pub use linux::get_system_metrics as get_system_metrics;
#[cfg(windows)]
pub use win::get_system_metrics as get_system_metrics;
#[cfg(mac)]
pub use mac::get_system_metrics as get_system_metrics;

