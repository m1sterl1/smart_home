/// Provides Transport trait and UDP and TCP types
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket},
    sync::{Arc, RwLock},
};

use crate::{
    command::{CommandRequest, CommandResponse},
    device::Device,
    errors::NetworkError,
    BUFLEN,
};

type Result<T> = std::result::Result<T, NetworkError>;

pub type SharedDevice = Arc<RwLock<dyn Device + Send + Sync>>;

/// Clue for UDP and TCP sockets
/// Each socket could receive CommandRequest, redirect it to NetworkDevice
/// and send CommandResponse back
pub trait Server: Sized {
    fn new<A: ToSocketAddrs>(addr: A) -> Result<Self>;
    fn listen(&self, device: SharedDevice) -> Result<()>;
}

/// Single threaded listener
/// One time connection: each client could use connection
/// only once
pub struct TCPServer {
    listener: TcpListener,
}

impl TCPServer {
    /// Handle "one time" connection
    fn handle(mut con: TcpStream, device: SharedDevice) -> Result<()> {
        // receive CommandRequest
        while let Ok(request) = Self::receive(&mut con) {
            // obtain NetworkDevice
            let mut device = device.write().unwrap();
            // process CommandRequest
            let resp = device.process(request);
            // send CommandResponse back
            Self::send(&mut con, resp)?;
        }
        Ok(())
    }

    /// Receive CommandRequest from tcp stream
    /// receiver buffer is limited with BUFLEN
    fn receive(con: &mut TcpStream) -> Result<CommandRequest> {
        let mut buf = vec![0u8; BUFLEN];
        let size = con.read(&mut buf)?;
        let req = CommandRequest::request_from(&buf[0..size])?;
        Ok(req)
    }

    /// Send CommandResponse back
    fn send(con: &mut TcpStream, resp: CommandResponse) -> Result<()> {
        let buf: Vec<u8> = resp.into();
        con.write_all(&buf)?;
        Ok(())
    }
}

impl Server for TCPServer {
    fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }
    fn listen(&self, device: SharedDevice) -> Result<()> {
        for con in self.listener.incoming() {
            let con = con?;
            Self::handle(con, device.clone())?;
        }
        Ok(())
    }
}

pub struct UDPServer {
    socket: UdpSocket,
}

impl UDPServer {
    // Receive CommandRequest from socket
    fn receive(&self) -> Result<(SocketAddr, CommandRequest)> {
        let mut buf = vec![0u8; BUFLEN];
        let (size, addr) = self.socket.recv_from(&mut buf)?;
        let req = CommandRequest::request_from(&buf[0..size])?;
        Ok((addr, req))
    }

    /// Send CommandResponse to addr
    fn send<A: ToSocketAddrs>(&self, resp: CommandResponse, addr: A) -> Result<()> {
        let buf: Vec<u8> = resp.into();
        self.socket.send_to(&buf, addr)?;
        Ok(())
    }

    fn handle(&self, device: SharedDevice) -> Result<()> {
        let (addr, request) = self.receive()?;
        let mut device = device.write().unwrap();
        let response = device.process(request);
        self.send(response, addr)?;
        Ok(())
    }
}

impl Server for UDPServer {
    fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }
    fn listen(&self, device: SharedDevice) -> Result<()> {
        loop {
            self.handle(device.clone())?;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::{Client, TCPClient, UDPClient};
    use smart_home::devices::Thermometer;
    use std::thread;

    use super::*;
    #[test]
    fn test_tcp_listener() {
        let listener = TCPServer::new("127.0.0.1:8010").unwrap();
        let device = Arc::new(RwLock::new(Thermometer::new("123")));
        let _t = thread::spawn(move || listener.listen(device));

        let mut s = TCPClient::new("127.0.0.1:8010").unwrap();
        s.send(CommandRequest::builder().therm("123").get_temp())
            .unwrap();

        let resp = s.receive().unwrap();
        println!("{resp:?}");
    }

    #[test]
    fn test_udp_listener() {
        let listener = UDPServer::new("127.0.0.1:8011").unwrap();
        let device = Arc::new(RwLock::new(Thermometer::new("123")));
        let _t = thread::spawn(move || listener.listen(device));

        let mut s = UDPClient::new("127.0.0.1:8011").unwrap();
        s.send(CommandRequest::builder().therm("123").get_temp())
            .unwrap();

        let resp = s.receive().unwrap();
        println!("{resp:?}");
    }
}
