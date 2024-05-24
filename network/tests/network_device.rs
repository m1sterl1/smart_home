use std::net::{TcpListener, UdpSocket};
use std::thread;
use network::command;
use network::transport::client::Client;
use network::{Result,
    network_device::NetworkDevice, 
    transport::{
        listener::{TCPListener, UDPListener},
        client::{TCPClient, UDPClient}
    },
    
    };
use smart_home::devices::*;



fn run() -> Result<()>{
    println!("start");
    let socket1 = Socket::new("s1000");
    let socket2 = Socket::new("s1001");
    let socket1_tcp:NetworkDevice<TCPListener> = NetworkDevice::new(socket1, "127.0.0.1:8000")?;
    let socket2_udp:NetworkDevice<UDPListener> = NetworkDevice::new(socket2, "127.0.0.1:8001")?;

    thread::spawn(move||socket1_tcp.listen());
    thread::spawn(move||socket2_udp.listen());

    let mut tcp_client = TCPClient::new("127.0.0.1:8000")?;
    let mut udp_client = UDPClient::new("127.0.0.1:8001")?;
    
    tcp_client.send(command::CommandRequest::new().socket("s1000").get_state())?;
    println!("Response tcp {:?}", tcp_client.receive());

    udp_client.send(command::CommandRequest::new().socket("s1001").get_state())?;
    println!("Response udp {:?}", udp_client.receive());

    Ok(())
}

#[test]
fn main(){
    if let Err(e) = run(){
        println!("Error {e}");
    }
}