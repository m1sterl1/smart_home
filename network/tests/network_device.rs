use network::{
    client::{Client, TCPClient, UDPClient},
    command::CommandRequest,
    network_device::NetworkDevice,
    transport::{TCP, UDP},
    Result,
};
use smart_home::devices::*;
use std::thread;

/// Send command with client, get and print response
fn send<C: Client>(client: &mut C, command: CommandRequest) -> Result<()> {
    client.send(command)?;
    let response = client.receive();
    println!("Response {response:?}");
    Ok(())
}

fn run() -> Result<()> {
    println!("start");
    let socket1 = Socket::new("s1000");
    let socket2 = Socket::new("s1001");
    let socket1_tcp: NetworkDevice<TCP> = NetworkDevice::new(socket1, "127.0.0.1:8000")?;
    let socket2_udp: NetworkDevice<UDP> = NetworkDevice::new(socket2, "127.0.0.1:8001")?;

    thread::spawn(move || socket1_tcp.listen());
    thread::spawn(move || socket2_udp.listen());

    let mut tcp_client = TCPClient::new("127.0.0.1:8000")?;
    let mut udp_client = UDPClient::new("127.0.0.1:8001")?;

    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").get_state(),
    )?;
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").turn_on(),
    )?;
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").get_state(),
    )?;

    send(
        &mut udp_client,
        CommandRequest::builder().socket("s1001").get_state(),
    )?;
    send(
        &mut udp_client,
        CommandRequest::builder().socket("s1001").turn_on(),
    )?;
    send(
        &mut udp_client,
        CommandRequest::builder().socket("s1001").get_state(),
    )?;

    Ok(())
}

#[test]
fn main() {
    if let Err(e) = run() {
        println!("Error {e}");
    }
}
