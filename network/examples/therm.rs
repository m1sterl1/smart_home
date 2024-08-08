use network::{
    sync::{Client, UDPClient, UDPServer},
    command::CommandRequest,
    network_device::NetworkDevice,
    Result,
};
use smart_home::devices::*;
use std::{thread, time::Duration};

/// Send command with client, get and print response
fn send<C: Client>(client: &mut C, command: CommandRequest) -> Result<()> {
    client.send(command)?;
    let response = client.receive();
    println!("Response {response:?}");
    Ok(())
}

fn run() -> Result<()> {
    // Starting network device
    let therm1 = Thermometer::new("t1000");
    let therm1_udp: NetworkDevice<UDPServer> = NetworkDevice::new(therm1, "127.0.0.1:8001")?;
    thread::spawn(move || therm1_udp.listen());

    // Init client
    let mut udp_client = UDPClient::new("127.0.0.1:8001")?;

    for _ in 0..5 {
        send(
            &mut udp_client,
            CommandRequest::builder().therm("t1000").get_temp(),
        )?;
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error {e}");
    }
}
