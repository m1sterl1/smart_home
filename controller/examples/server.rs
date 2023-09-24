use controller::server::Controller;

fn main() {
    // Create SocetServer
    let server = Controller::new("127.0.0.1:10221").unwrap();
    // Create (register) "First" socket on srever side
    server.add_socket("First");
    server.listen();
}
