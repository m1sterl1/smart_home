use std::fmt::Display;

use smart_home::{devices::Socket, sources::DeviceSource, SmartHome};

#[test]
fn test_report() {
    let socket1 = Socket::new("Smart Socket v1.0");
    let mut house = SmartHome::new("City home");
    house.add_device("guestroom", "Thermometer1").unwrap();
    house.add_device("bedroom", "Socket1").unwrap();

    let mut info_provider1 = DeviceSource::new();
    info_provider1
        .add_device("Socket1", "bedroom", &socket1 as &dyn Display)
        .unwrap();
    let report1 = house.create_report(&info_provider1).unwrap();

    assert_eq!(
        report1.to_string(),
        "Report for City home smart home

bedroom             Socket1             State: off, power consumption 0.0W
guestroom           Thermometer1        Error connecting device
"
    )
}
