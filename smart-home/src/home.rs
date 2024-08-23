use std::collections::{HashMap, HashSet};

use serde::Serialize;
use thiserror::Error;

use crate::DeviceInfoProvider;

#[derive(Debug, Error, PartialEq)]
pub enum SmartHomeError {
    #[error("Room {0} already exists")]
    DupRoom(String), // Duplicate room
    #[error("No such room {0}")]
    NoRoom(String), // No such room
    #[error("Device {device} already exists in the room {room}")]
    DupDevice { room: String, device: String }, // Duplicate device in the room
    #[error("No device {device} in the room {room}")]
    NoDevice { room: String, device: String }, // No device in the room with such name
    #[error("No rooms in home")]
    NoRooms, // No rooms in home
}

impl Serialize for SmartHomeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Unique room name with unique devices
type Rooms = HashMap<String, HashSet<String>>;

/// Smart home contains rooms
/// Room has unique name
pub struct SmartHome {
    name: String,
    rooms: Rooms,
}

impl SmartHome {
    /// Creates SmartHome instance with empty rooms
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::default(),
        }
    }

    /// Register room in the home
    /// If room name exists, error is returned
    pub fn add_room(&mut self, room_name: &str) -> Result<(), SmartHomeError> {
        match self.rooms.get(room_name) {
            Some(_) => Err(SmartHomeError::DupRoom(room_name.to_string())),
            None => {
                self.rooms.insert(room_name.to_string(), HashSet::default());
                Ok(())
            }
        }
    }

    /// Remove room from home with all associated devices
    /// If no room exists error is returned
    pub fn remove_room(&mut self, room_name: &str) -> Result<(), SmartHomeError> {
        match self.rooms.remove(room_name) {
            Some(_) => Ok(()),
            None => Err(SmartHomeError::NoRoom(room_name.to_string())),
        }
    }

    /// Register device name in the room
    /// If there is no room, new room will be created
    /// Returns error if device exists
    pub fn add_device(&mut self, room_name: &str, dev_name: &str) -> Result<(), SmartHomeError> {
        match self.rooms.get_mut(room_name) {
            Some(rooms) => {
                if rooms.insert(dev_name.to_string()) {
                    Ok(())
                } else {
                    Err(SmartHomeError::DupDevice {
                        room: room_name.to_string(),
                        device: dev_name.to_string(),
                    })
                }
            }
            None => {
                self.rooms
                    .insert(room_name.to_string(), [dev_name.to_string()].into());
                Ok(())
            }
        }
    }

    /// Remove device from a room
    /// If there is not device or room with specified names error is returned
    pub fn remove_device(&mut self, room_name: &str, dev_name: &str) -> Result<(), SmartHomeError> {
        match self.rooms.get_mut(room_name) {
            Some(room) => {
                if room.remove(dev_name) {
                    Ok(())
                } else {
                    Err(SmartHomeError::NoDevice {
                        room: room_name.to_string(),
                        device: dev_name.to_string(),
                    })
                }
            }
            None => Err(SmartHomeError::NoRoom(room_name.to_string())),
        }
    }

    /// Get rooms list
    pub fn get_rooms(&self) -> Vec<&str> {
        let mut rooms = self
            .rooms
            .keys()
            .map(|name| name.as_str())
            .collect::<Vec<_>>();
        rooms.sort_unstable();
        rooms
    }

    /// Get devices in the room
    pub fn devices(&self, room: &str) -> Result<Vec<&str>, SmartHomeError> {
        match self.rooms.get(room) {
            Some(devices) => {
                let mut devices = devices.iter().map(|d| d.as_str()).collect::<Vec<_>>();
                devices.sort_unstable();
                Ok(devices)
            }
            None => Err(SmartHomeError::NoRoom(room.to_string())),
        }
    }

    /// Create report based on type implementing DeviceInfoProvider trait
    pub fn create_report<T: DeviceInfoProvider>(
        &self,
        info_provider: &T,
    ) -> Result<String, SmartHomeError> {
        let mut report = format!("Report for {} smart home\n\n", self.name);
        match self.get_rooms() {
            rooms if !rooms.is_empty() => {
                for room in self.get_rooms() {
                    for device in self.devices(room).unwrap() {
                        let line = format!("{}\n", info_provider.get_info(room, device));
                        report.push_str(&line);
                    }
                }
                Ok(report)
            }
            _ => Err(SmartHomeError::NoRooms),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{SmartHome, SmartHomeError};

    #[test]
    fn test_rooms() {
        let mut h = SmartHome::new("My home");
        h.add_room("Guestroom").unwrap();
        h.add_room("Bathroom").unwrap();
        assert_eq!(h.get_rooms(), vec!["Bathroom", "Guestroom"])
    }

    #[test]
    fn test_devices() {
        let mut h = SmartHome::new("My home");
        h.add_room("Guestroom").unwrap();
        h.add_device("Guestroom", "Thermometer").unwrap();
        h.add_device("Guestroom", "Socket").unwrap();
        assert_eq!(
            h.devices("Guestroom").unwrap(),
            vec!["Socket", "Thermometer"]
        )
    }

    #[test]
    /// Try to add room with existing name returns error
    fn test_add_room() {
        let mut h = SmartHome::new("My home");
        // Ok if first room
        assert_eq!(h.add_room("Oneroom"), Ok(()));
        // Error if name already exists
        assert_eq!(
            h.add_room("Oneroom"),
            Err(SmartHomeError::DupRoom("Oneroom".to_string()))
        );
    }

    #[test]
    /// Try to add room with existing name returns error
    fn test_remove_room() {
        let mut h = SmartHome::new("My home");
        h.add_room("Oneroom").unwrap();
        assert_eq!(h.remove_room("Oneroom"), Ok(()));
        // Error if no such room
        assert_eq!(
            h.remove_room("Oneroom"),
            Err(SmartHomeError::NoRoom("Oneroom".to_string()))
        );
    }

    #[test]
    /// Try to add device with existing name within a room returns error
    fn test_add_device() {
        let mut h = SmartHome::new("My home");
        // Ok if first device
        assert_eq!(h.add_device("Oneroom", "mydevice"), Ok(()));
        // Error if name already exists
        assert_eq!(
            h.add_device("Oneroom", "mydevice"),
            Err(SmartHomeError::DupDevice {
                device: "mydevice".to_string(),
                room: "Oneroom".to_string()
            })
        );
    }

    #[test]
    fn test_remove_device() {
        let mut h = SmartHome::new("My home");
        h.add_device("Oneroom", "mydevice").unwrap();
        assert_eq!(h.remove_device("Oneroom", "mydevice"), Ok(()));
        // if no such device return error
        assert_eq!(
            h.remove_device("Oneroom", "mydevice"),
            Err(SmartHomeError::NoDevice {
                room: "Oneroom".to_string(),
                device: "mydevice".to_string()
            })
        )
    }
}
