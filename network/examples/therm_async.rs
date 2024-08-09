use network::{
    command::CommandRequest,
    r#async::{ClientAsync, NetworkDeviceAsync, UDPClientAsync, UDPServerAsync},
    Result,
};
use smart_home::devices::*;
use std::time::Duration;
use tokio::{runtime::Runtime, time::sleep};

/// Send command with client, get and print response
async fn send<C: ClientAsync>(client: &mut C, command: CommandRequest) -> Result<()> {
    client.send(command).await?;
    let response = client.receive().await;
    println!("Response {response:?}");
    Ok(())
}

async fn run() -> Result<()> {
    // Starting network device
    let therm1 = Thermometer::new("t1000");
    let therm1_udp: NetworkDeviceAsync<UDPServerAsync> =
        NetworkDeviceAsync::new(therm1, "127.0.0.1:8001").await?;
    tokio::spawn(async move { therm1_udp.listen().await });

    // Init client
    let mut udp_client = UDPClientAsync::new("127.0.0.1:8001").await?;

    for _ in 0..5 {
        send(
            &mut udp_client,
            CommandRequest::builder().therm("t1000").get_temp(),
        )
        .await?;
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

fn main() {
    let result = Runtime::new().unwrap().block_on(run());
    if let Err(e) = result {
        println!("Error {e}");
    }
}
