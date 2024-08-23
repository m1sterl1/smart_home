use std::time::Duration;

use actix_web::rt::{spawn, time};

use web::{server_run, Result, SmartHomeClient};

const SERVER_BIND: &str = "127.0.0.1:8080";
const CLIENT_BASE: &str = "http://127.0.0.1:8080";

#[actix_web::main]
async fn main() -> Result<()> {
    spawn(server_run(SERVER_BIND));
    time::sleep(Duration::from_secs(1)).await;

    let client = SmartHomeClient::new(CLIENT_BASE)?;

    println!("Rooms: {}", client.rooms().await?);
    println!("Add room: {}", client.rooms_add("Guestroom").await?);
    println!("Add room: {}", client.rooms_add("Bathroom").await?);
    println!("Rooms: {}", client.rooms().await?);
    println!("Remove room: {}", client.rooms_del("Kitchen").await?);

    println!(
        "Add device: {}",
        client.devices_add("Guestroom", "Socket").await?
    );
    println!(
        "Add device: {}",
        client.devices_add("Bathroom", "Thermometer").await?
    );
    println!("Devices Guestroom: {}", client.devices("Guestroom").await?);
    println!("Devices Bathroom: {}", client.devices("Bathroom").await?);
    println!("Devices Kitchen: {}", client.devices("Kitchen").await?);

    println!("Report: {}", client.report().await?);

    Ok(())
}
