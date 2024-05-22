use std::{
    error::Error,
    fmt::{Display, Write},
    process::exit,
};

use smart_home::{
    devices::{Socket, Thermometer, ThermometerState},
    sources::DeviceSource,
    SmartHome, SmartHomeError,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Initialize home with devices
fn init_house(home: &mut SmartHome) -> std::result::Result<(), SmartHomeError> {
    home.add_device("bedroom", "Socket1")?;
    home.add_device("guestroom", "Socket2")?;
    home.add_device("kitchen", "Thermometer1")?;
    Ok(())
}

/// Create report with devices list
fn report_devices(home: &SmartHome) -> std::result::Result<String, SmartHomeError> {
    // Get list of devices and rooms in the house
    let mut devices_list = String::new();
    for room in home.get_rooms() {
        writeln!(&mut devices_list, "{room}:").unwrap();
        for device in home.devices(room)? {
            writeln!(&mut devices_list, "\t{device}").unwrap();
        }
    }
    Ok(devices_list)
}

fn start() -> Result<()> {
    // Device initialization
    let mut socket1 = Socket::new("Smart Socket v1.0");
    socket1.turn_on()?;
    let socket2 = Socket::new("Smart Socket v1.0");
    let thermo = Thermometer::new("Thermometer v1.0") ;

    //////// SmartHome initialization ////////
    let mut home = SmartHome::new("City home");

    init_house(&mut home)?;

    // //////// Sources initialization ////////
    // Info provider which borrow devices as &dyn Display trait object
    let mut info_provider1 = DeviceSource::new();
    info_provider1.add_device("Socket1", "bedroom", &socket1 as &dyn Display)?;
    info_provider1.add_device("Thermometer1", "kitchen", &thermo as &dyn Display)?;
    let report1 = home.create_report(&info_provider1)?;

    // Info provider which owns devices as Box<dyn Display> trait object
    let mut info_provider2 = DeviceSource::new();
    info_provider2.add_device(
        "Socket2",
        "guestroom",
        Box::new(socket2) as Box<dyn Display>,
    )?;
    let report2 = home.create_report(&info_provider2)?;

    let devices_list = report_devices(&home)?;

    // Report on the screen
    println!("Rooms and devices:\n{devices_list}");
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
    Ok(())
}

fn main() {
    start().unwrap_or_else(|e| {
        println!("{e}");
        exit(1)
    });
}
