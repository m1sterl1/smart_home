/// Module provides Client trait and Clients for TCP and UDP protocols
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs, UdpSocket},
};

use crate::{
    command::{CommandRequest, CommandResponse},
    Result, BUFLEN,
};

/// Client which unite TCP and UDP sockets
pub trait Client {
    fn send(&mut self, request: CommandRequest) -> Result<()>;
    fn receive(&mut self) -> Result<CommandResponse>;
    fn get(&mut self, request: CommandRequest) -> Result<CommandResponse> {
        self.send(request)?;
        self.receive()
    }
}

pub struct TCPClient {
    stream: TcpStream,
}

impl TCPClient {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }
}

impl Client for TCPClient {
    fn send(&mut self, request: CommandRequest) -> Result<()> {
        let buf = serde_json::to_vec(&request)?;
        self.stream.write_all(&buf)?;
        Ok(())
    }

    fn receive(&mut self) -> Result<CommandResponse> {
        let mut buf = vec![0; BUFLEN];
        let size = self.stream.read(&mut buf)?;
        let resp: CommandResponse = serde_json::from_slice(&buf[0..size])?;
        Ok(resp)
    }
}

pub struct UDPClient {
    socket: UdpSocket,
}

impl UDPClient {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }
}

impl Client for UDPClient {
    fn send(&mut self, request: CommandRequest) -> Result<()> {
        let buf = serde_json::to_vec(&request)?;
        self.socket.send(&buf)?;
        Ok(())
    }
    fn receive(&mut self) -> Result<CommandResponse> {
        let mut buf = vec![0; BUFLEN];
        let size = self.socket.recv(&mut buf)?;
        let resp: CommandResponse = serde_json::from_slice(&buf[0..size])?;
        Ok(resp)
    }
}
