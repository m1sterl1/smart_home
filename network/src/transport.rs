use std::{error::Error, io::Read, net::{TcpListener, TcpStream, UdpSocket}, net::SocketAddr, sync::Arc, thread};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

const BUFLEN: usize = 1024;

struct TCPListener{
    listener: TcpListener,
}

struct UDPListener{
    socket: Arc<UdpSocket>,
}

impl TCPListener {
    fn handle(mut con: TcpStream) -> Result<()>{
        let mut buf = vec![0u8;BUFLEN];
        con.read(&mut buf)?;
        let s = String::from_utf8(buf)?;
        // serialize to command
        // perform action
        // send response
        Ok(())
    } 
}

impl UDPListener {
    fn handle(s: Arc<UdpSocket>, buf: Vec<u8>, address: SocketAddr) -> Result<()>{
        // serialize to command
        // perform action
        // send response 
        Ok(())
    } 
}

trait Listener{
    fn listen(&self) -> Result<()>;
}

impl Listener for TCPListener{
    fn listen(&self) -> Result<()>{
        for con in self.listener.incoming(){
            let con = con?;
            thread::spawn(move ||Self::handle(con));
        }
        Ok(())
    }
}

impl Listener for UDPListener{
    fn listen(&self) -> Result<()> {
        loop {
            let mut buf = vec![0u8;BUFLEN];
            let (_, addr) = self.socket.recv_from(&mut buf)?;
            let s = self.socket.clone();
            thread::spawn(move||Self::handle(s, buf, addr));
        }
    }
}