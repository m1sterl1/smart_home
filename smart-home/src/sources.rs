/// Device sources
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::DeviceInfoProvider;

//   Id = (Name,   Room  )
type Id = (String, String);
type Devices<T> = HashMap<Id, T>;

/// Type which contains devices
/// Devices added to source are cached in the hashmap by (name, room)
/// Device could be any type which implements Display trait
pub struct DeviceSource<T: Display> {
    devices: Devices<T>,
}
#[derive(Debug, PartialEq)]
pub enum DeviceSourceError {
    DupDevice { device: String, room: String },
}

impl Display for DeviceSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DupDevice { device, room } => {
                write!(f, "Device {} already exists in the room {}", device, room)
            }
        }
    }
}

impl Error for DeviceSourceError {}

impl<T: Display> DeviceSource<T> {
    /// Creates DeviceSource with empty table
    pub fn new() -> Self {
        Self {
            devices: Devices::default(),
        }
    }
    /// Insert device into table
    pub fn add_device(
        &mut self,
        device_name: &str,
        room: &str,
        device: T,
    ) -> Result<(), DeviceSourceError> {
        let key = (device_name.to_string(), room.to_string());
        match self.devices.get(&key) {
            Some(_) => Err(DeviceSourceError::DupDevice {
                device: key.0,
                room: key.1,
            }),
            None => {
                self.devices
                    .insert((device_name.to_string(), room.to_string()), device);
                Ok(())
            }
        }
    }
}

impl<T: Display> DeviceInfoProvider for DeviceSource<T> {
    fn get_info(&self, room: &str, device: &str) -> String {
        match self.devices.get(&(device.to_string(), room.to_string())) {
            Some(d) => format!("{room:<20}{device:<20}{d}"),
            None => format!("{room:<20}{device:<20}Error connecting device"),
        }
    }
}

impl<T: Display> Default for DeviceSource<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::DeviceSource;
    use crate::{sources::DeviceSourceError, DeviceInfoProvider};

    #[test]
    /// If device exists return error
    fn test_add_device() {
        let mut s = DeviceSource::new();
        let result = s.add_device("Socket-1", "Guestroom", "Socket-1 state description");
        assert_eq!(result, Ok(()));
        let result = s.add_device("Socket-1", "Guestroom", "Socket-1 state description");
        assert_eq!(
            result,
            Err(DeviceSourceError::DupDevice {
                device: "Socket-1".to_string(),
                room: "Guestroom".to_string()
            })
        );
    }

    #[test]
    fn test_get_info() {
        let mut s = DeviceSource::new();
        s.add_device("Socket-1", "Guestroom", "Socket-1 state description")
            .unwrap();

        assert_eq!(
            s.get_info("Guestroom", "Socket-1"),
            "Guestroom           Socket-1            Socket-1 state description"
        );
        assert_eq!(
            s.get_info("Kitchen", "Socket-2"),
            "Kitchen             Socket-2            Error connecting device"
        );
    }
}
