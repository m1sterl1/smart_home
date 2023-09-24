use controller::server::Controller;

fn main() {
    // Create SocetServer
    let mut server = Controller::new("127.0.0.1:10221").unwrap();
    // Create (register) "First" socket on srever side
    server.add_socket("First");
    match server.listen() {
        Err(e) => println!("{e}"),
        _ => {}
    }
}
