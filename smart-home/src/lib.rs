pub mod devices;
mod home;
pub mod sources;

pub use home::{SmartHome, SmartHomeError};

/// Interface for container with live devices
pub trait DeviceInfoProvider {
    /// Method returns device info from room name and device name
    fn get_info(&self, room: &str, device: &str) -> String;
}
