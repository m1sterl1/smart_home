use tokio::runtime::Runtime;

use network::{
    command::CommandRequest,
    r#async::{ClientAsync, NetworkDeviceAsync, TCPClientAsync, TCPServerAsync},
    Result,
};
use smart_home::devices::*;

/// Send command with client, get and print response
async fn send<C: ClientAsync>(client: &mut C, command: CommandRequest) -> Result<()> {
    client.send(command).await?;
    let response = client.receive().await;
    println!("Response {response:?}");
    Ok(())
}

async fn run() -> Result<()> {
    // Starting network device
    let socket1 = Socket::new("s1000");
    let socket1_tcp: NetworkDeviceAsync<TCPServerAsync> =
        NetworkDeviceAsync::new(socket1, "127.0.0.1:8000").await?;

    tokio::spawn(async move { socket1_tcp.listen().await });

    let mut tcp_client = TCPClientAsync::new("127.0.0.1:8000").await?;

    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").get_state(),
    )
    .await?;
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").turn_on(),
    )
    .await?;
    send(
        &mut tcp_client,
        CommandRequest::builder().socket("s1000").get_state(),
    )
    .await?;

    Ok(())
}

fn main() {
    let result = Runtime::new().unwrap().block_on(run());
    if let Err(e) = result {
        println!("Error {e}");
    }
}
