use controller::server::SocketServer;

fn main() {
    // Create SocetServer
    let mut server = SocketServer::new("127.0.0.1:10221").unwrap();
    // Create (register) "First" socket on srever side
    server.add_socket("First");
    match server.listen() {
        Err(e) => println!("{e}"),
        _ => {}
    }
}
