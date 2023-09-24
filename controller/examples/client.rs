use std::{thread, time::Duration};

use controller::client::SocketCLient;

const SLEEP:Duration = Duration::from_secs(1);

fn start_client(){
        // Create client
        let mut client = SocketCLient::new("127.0.0.1:10221").unwrap();
        // Turn on "First" socket
        let response = client.turn_on("First");
        println!("{response:?}");
        thread::sleep(SLEEP);

        // Get state from "First" socket
        let response = client.get_state("First");
        println!("{response:?}");
        thread::sleep(SLEEP);

        // Get state from "Second" socket
        let response = client.get_state("Second");
        println!("{response:?}");
        thread::sleep(SLEEP);
        // Trun off "First" socket

        let response = client.turn_off("First");
        println!("{response:?}");
}

fn main() {
    let mut handles = Vec::new();
    for _ in 0..2{
        handles.push(thread::spawn(||start_client()));
    }
    for handle in handles{
        handle.join().unwrap();
    }
}
