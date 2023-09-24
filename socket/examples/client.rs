use socket::client::SocketCLient;

fn main(){
    // Create client
    let mut client = SocketCLient::new("127.0.0.1:10221").unwrap();
    // Turn on "First" socket
    let response = client.turn_on("First");
    println!("{response:?}");
    // Get state from "First" socket
    let response = client.get_state("First");
    println!("{response:?}");
    // Get state from "Second" socket
    let response = client.get_state("Second");
    println!("{response:?}");
    // Trun off "First" socket
    let response = client.turn_off("First");
    println!("{response:?}");
}