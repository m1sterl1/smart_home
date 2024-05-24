use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use std::thread::{self, sleep, spawn};
use std::time::Duration;

fn handle<A: ToSocketAddrs>(socket: Arc<UdpSocket>, addr: A, val: usize) -> std::io::Result<()> {
    sleep(Duration::from_secs(1));
    let val = val + 1;
    socket.send_to(&val.to_be_bytes(), addr)?;
    println!("Handler: sent {val}");
    Ok(())
}

fn server() -> std::io::Result<()> {
    let s = UdpSocket::bind("127.0.0.1:20000").unwrap();
    let s = Arc::new(s);
    loop {
        let mut buf = [0; 8];
        println!("Server: wating for datagramms");
        let (_, addr) = s.recv_from(&mut buf)?;
        println!("Server: got val from {addr}");
        let socket = s.clone();
        let val = usize::from_be_bytes(buf);
        spawn(move || handle(socket, addr, val));
    }
}

#[test]
fn main() {
    println!("main");
    let t = thread::spawn(server);
    sleep(Duration::from_secs(1));
    let s = UdpSocket::bind("127.0.0.1:30000").unwrap();
    let value: usize = 100;
    for _ in 0..2 {
        let mut buf = value.to_be_bytes();
        println!("Client: sending value {value}");
        s.send_to(&buf, "127.0.0.1:20000").unwrap();
        s.recv(&mut buf).unwrap();
        println!("Client: got buffer {:?}", buf);
    }
    t.join().unwrap().unwrap();
}
