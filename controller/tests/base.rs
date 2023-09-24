use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use controller::{client::SocketCLient, command::CommandResponse, server::Controller};

const ADDR: &str = "127.0.0.1:10221";

fn run_server(addr: &str) {
    let server = Controller::new(addr).unwrap();
    server.add_socket("First");
    server.listen()
}

#[test]
fn main() {
    // Run server
    spawn(|| run_server(ADDR));
    sleep(Duration::from_secs(1));

    // Create client
    let mut client = SocketCLient::new(ADDR).unwrap();

    // Turn on
    let response = client.turn_on("First").unwrap();
    assert_eq!(response, CommandResponse::SocketTurnOn(Ok(())));

    // Get state
    let response = client.get_state("First").unwrap();
    // compare only beginning of messages (message from server contains random value)
    match response {
        CommandResponse::SocketGetState(Ok(s)) => assert!(s.starts_with("State: on, power consumption")),
        _ => assert!(false),
    }

    // Get state from non existing device
    let response = client.get_state("Second").unwrap();
    assert_eq!(
        response,
        CommandResponse::SocketGetState(Err("No socket".to_string()))
    );

    // Turn off socket and get state
    client.turn_off("First").unwrap();
    let response = client.get_state("First").unwrap();
    assert_eq!(
        response,
        CommandResponse::SocketGetState(Ok("State: off, power consumption 0.0W".to_string()))
    );
}
