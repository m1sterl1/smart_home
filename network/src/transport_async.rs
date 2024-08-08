/// Provides Transport trait and UDP and TCP types
use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket},
    sync::RwLock,
};

use crate::{
    command::{CommandRequest, CommandResponse},
    device::Device,
    Result, BUFLEN,
};

pub type SharedDevice = Arc<RwLock<dyn Device + Send + Sync>>;

/// Clue for UDP and TCP sockets
/// Each socket could receive CommandRequest, redirect it to NetworkDevice
/// and send CommandResponse back
pub trait Transport: Sized {
    async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self>;
    async fn listen(&self, device: SharedDevice) -> Result<()>;
}

pub struct TCP {
    listener: TcpListener,
}

impl TCP {
    /// Handle "one time" connection
    async fn handle(mut con: TcpStream, device: SharedDevice) -> Result<()> {
        // receive CommandRequest
        while let Ok(request) = Self::receive(&mut con).await {
            // obtain NetworkDevice
            let mut device = device.write().await;
            // process CommandRequest
            let resp = device.process(request);
            // send CommandResponse back
            Self::send(&mut con, resp).await?;
        }
        Ok(())
    }

    /// Receive CommandRequest from tcp stream
    /// receiver buffer is limited with BUFLEN
    async fn receive(con: &mut TcpStream) -> Result<CommandRequest> {
        let mut buf = vec![0u8; BUFLEN];
        let size = con.read(&mut buf).await?;
        let req = CommandRequest::request_from(&buf[0..size])?;
        Ok(req)
    }

    /// Send CommandResponse back
    async fn send(con: &mut TcpStream, resp: CommandResponse) -> Result<()> {
        let buf: Vec<u8> = resp.into();
        con.write_all(&buf).await?;
        Ok(())
    }
}

impl Transport for TCP {
    async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }
    async fn listen(&self, device: SharedDevice) -> Result<()> {
        while let Ok((stream, _)) = self.listener.accept().await {
            Self::handle(stream, device.clone()).await?;
        }
        Ok(())
    }
}

pub struct UDP {
    socket: UdpSocket,
}

impl UDP {
    // Receive CommandRequest from socket
    async fn receive(&self) -> Result<(SocketAddr, CommandRequest)> {
        let mut buf = vec![0u8; BUFLEN];
        let (size, addr) = self.socket.recv_from(&mut buf).await?;
        let req = CommandRequest::request_from(&buf[0..size])?;
        Ok((addr, req))
    }

    /// Send CommandResponse to addr
    async fn send<A: ToSocketAddrs>(&self, resp: CommandResponse, addr: A) -> Result<()> {
        let buf: Vec<u8> = resp.into();
        self.socket.send_to(&buf, addr).await?;
        Ok(())
    }

    async fn handle(&self, device: SharedDevice) -> Result<()> {
        let (addr, request) = self.receive().await?;
        let mut device = device.write().await;
        let response = device.process(request);
        self.send(response, addr).await?;
        Ok(())
    }
}

impl Transport for UDP {
    async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self { socket })
    }
    async fn listen(&self, device: SharedDevice) -> Result<()> {
        loop {
            self.handle(device.clone()).await?;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client_async::{Client, TCPClient, UDPClient};
    use smart_home::devices::Thermometer;

    use super::*;
    #[tokio::test]
    async fn test_tcp_listener() {
        let listener = TCP::new("127.0.0.1:8008").await.unwrap();
        let device = Arc::new(RwLock::new(Thermometer::new("123")));
        let _t = tokio::spawn(async move { listener.listen(device).await });

        let mut s = TCPClient::new("127.0.0.1:8008").await.unwrap();
        s.send(CommandRequest::builder().therm("123").get_temp())
            .await
            .unwrap();

        let resp = s.receive().await.unwrap();
        println!("{resp:?}");
    }

    #[tokio::test]
    async fn test_udp_listener() {
        let listener = UDP::new("127.0.0.1:8008").await.unwrap();
        let device = Arc::new(RwLock::new(Thermometer::new("123")));
        let _t = tokio::spawn(async move { listener.listen(device).await });

        let mut s = UDPClient::new("127.0.0.1:8008").await.unwrap();
        s.send(CommandRequest::builder().therm("123").get_temp())
            .await
            .unwrap();

        let resp = s.receive().await.unwrap();
        println!("{resp:?}");
    }
}
