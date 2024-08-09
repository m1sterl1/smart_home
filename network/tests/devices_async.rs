use network::{
    command::CommandRequest,
    r#async::{TCPServerAsync, UDPServerAsync, ClientAsync, TCPClientAsync, UDPClientAsync, NetworkDeviceAsync},
    Result,
};
use smart_home::devices::*;


/// Send command with client, get and print response
async fn send<C: ClientAsync>(client: &mut C, command: CommandRequest) -> Result<()> {
    client.send(command).await?;
    let response = client.receive().await?;
    println!("Response {response:?}");
    Ok(())
}

/// Test two async sockets with different transports: TCP and UDP
async fn sockets() -> Result<()> {
    println!("start");
    // Sockets devices
    let socket1 = Socket::new("s1000");
    let socket2 = Socket::new("s1001");
    // Wrap with NetworkDevice with different transports
    let socket1_tcp: NetworkDeviceAsync<TCPServerAsync> = NetworkDeviceAsync::new(socket1, "127.0.0.1:8000").await?;
    let socket2_udp: NetworkDeviceAsync<UDPServerAsync> = NetworkDeviceAsync::new(socket2, "127.0.0.1:8001").await?;
    // Run listeners (servers)
    tokio::spawn(async move {socket1_tcp.listen().await});
    tokio::spawn(async move {socket2_udp.listen().await});
    // Clients for network devices
    let mut tcp_client = TCPClientAsync::new("127.0.0.1:8000").await?;
    let mut udp_client = UDPClientAsync::new("127.0.0.1:8001").await?;
    // Send different requests
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").get_state(),
    ).await?;
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").turn_on(),
    ).await?;
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").get_state(),
    ).await?;

    send(
        &mut udp_client,
        CommandRequest::builder().socket("s1001").get_state(),
    ).await?;
    send(
        &mut udp_client,
        CommandRequest::builder().socket("s1001").turn_on(),
    ).await?;
    send(
        &mut udp_client,
        CommandRequest::builder().socket("s1001").get_state(),
    ).await?;

    Ok(())
}

#[tokio::test]
async fn main() {
    if let Err(e) = sockets().await {
        println!("Error {e}");
    }
}
