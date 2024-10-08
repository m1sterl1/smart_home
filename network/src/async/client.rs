/// Module provides Client trait and Clients for TCP and UDP protocols
use std::net::{SocketAddr, ToSocketAddrs};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpSocket, TcpStream, UdpSocket},
};

use crate::{
    command::{CommandRequest, CommandResponse},
    Result, BUFLEN,
};

/// Client which unite TCP and UDP sockets
pub trait ClientAsync {
    fn send(
        &mut self,
        request: CommandRequest,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn receive(&mut self) -> impl std::future::Future<Output = Result<CommandResponse>> + Send;
}

pub struct TCPClientAsync {
    stream: TcpStream,
}

impl TCPClientAsync {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = TcpSocket::new_v4()?;
        let addr = get_sock_addr(addr)?;
        let stream = socket.connect(addr).await?;
        Ok(Self { stream })
    }
}

impl ClientAsync for TCPClientAsync {
    async fn send(&mut self, request: CommandRequest) -> Result<()> {
        let buf = serde_json::to_vec(&request)?;
        self.stream.write_all(&buf).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<CommandResponse> {
        let mut buf = vec![0; BUFLEN];
        let size = self.stream.read(&mut buf).await?;
        let resp: CommandResponse = serde_json::from_slice(&buf[0..size])?;
        Ok(resp)
    }
}
pub struct UDPClientAsync {
    socket: UdpSocket,
}

impl UDPClientAsync {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        let addr = get_sock_addr(addr)?;
        socket.connect(addr).await?;
        Ok(Self { socket })
    }
}

impl ClientAsync for UDPClientAsync {
    async fn send(&mut self, request: CommandRequest) -> Result<()> {
        let buf = serde_json::to_vec(&request)?;
        self.socket.send(&buf).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<CommandResponse> {
        let mut buf = vec![0; BUFLEN];
        let size = self.socket.recv(&mut buf).await?;
        let resp: CommandResponse = serde_json::from_slice(&buf[0..size])?;
        Ok(resp)
    }
}

fn get_sock_addr<A: ToSocketAddrs>(addr: A) -> Result<SocketAddr> {
    addr.to_socket_addrs()?
        .next()
        .ok_or("Error converting to socket addr".into())
}
