use network::{
    sync::{Client, TCPClient, TCPServer},
    command::CommandRequest,
    network_device::NetworkDevice,
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
    // Starting network device
    let socket1 = Socket::new("s1000");
    let socket1_tcp: NetworkDevice<TCPServer> = NetworkDevice::new(socket1, "127.0.0.1:8000")?;

    thread::spawn(move || socket1_tcp.listen());

    let mut tcp_client = TCPClient::new("127.0.0.1:8000")?;

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

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error {e}");
    }
}
