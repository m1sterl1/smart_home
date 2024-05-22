use std::{io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket}, sync::{Arc, RwLock}};

use serde::Serialize;

use crate::{command::{CommandRequest, CommandResponse}, device::Device, Result};

const BUFLEN: usize = 1024;

type SharedDevice = Arc<RwLock<dyn Device>>;

struct TCPListener{
    listener: TcpListener,
}

impl TCPListener {

    pub fn new<A:ToSocketAddrs>(addr: A) -> Result<Self>{
        let listener = TcpListener::bind(addr)?;
        Ok(Self{listener})
    }

    fn handle(mut con: TcpStream, device: SharedDevice) -> Result<()>{
        let request = Self::receive(&mut con)?;
        println!("L: got req {request:?}");
        let mut device = device.write().unwrap();
        let resp = device.process(request);
        Self::send(&mut con, resp)?;
        Ok(())
    } 

    fn receive(con: &mut TcpStream) -> Result<CommandRequest>{
        let mut buf = vec![0u8;BUFLEN];
        let size = con.read(&mut buf)?;
        let req = CommandRequest::request_from(&buf[0..size])?;
        Ok(req)
    }

    fn send(con: &mut TcpStream, resp: CommandResponse) -> Result<()>{
        let buf:Vec<u8> = resp.into();
        con.write(&buf)?;
        Ok(())
    }


}
struct UDPListener{
    socket: Arc<UdpSocket>,
}

impl UDPListener {
    fn handle(s: Arc<UdpSocket>, buf: Vec<u8>, address: SocketAddr) -> Result<()>{
        // serialize to request object
        let request = CommandRequest::request_from(&buf)?;
        // perform action

        // send responseerialize to command
        // perform action
        // send response 
        Ok(())
    }
}


trait Listener{
    fn listen(&self, device: SharedDevice) -> Result<()>;
}

impl Listener for TCPListener{
    fn listen(&self, device: SharedDevice) -> Result<()>{
        for con in self.listener.incoming(){
            let con = con?;
            Self::handle(con, device.clone())?;
        }
        Ok(())
    }
}

// impl Listener for UDPListener{
//     fn listen(&self) -> Result<()> {
//         loop {
//             let mut buf = vec![0u8;BUFLEN];
//             let (_, addr) = self.socket.recv_from(&mut buf)?;
//             let s = self.socket.clone();
            
//         }
//     }
// }


#[cfg(test)]
mod tests{
    use std::{thread, time::Duration};

    use smart_home::devices::Thermometer;

    use super::*;
    #[test]
    fn test_tcp_listener(){
        let listener = TCPListener::new("127.0.0.1:8008").unwrap();
        let device = Arc::new(RwLock::new(Thermometer::new("123")));
        let t = thread::spawn(move||listener.listen(device));
        let mut s = TcpStream::connect("127.0.0.1:8008").unwrap();
        let request = CommandRequest::new().therm("123").get_temp();
        let buf = serde_json::to_vec(&request).unwrap();
        let size = s.write(&buf).unwrap();
        
        // thread::sleep(Duration::from_secs(2));
        let mut buf = vec![0;BUFLEN];
        let size = s.read(&mut buf).unwrap();
        println!("Buf of len {size} read");
        let resp:CommandResponse = serde_json::from_slice(&buf[0..size]).unwrap();
        println!("Response {resp:?}");
        // let r = t.join().unwrap();
        // println!("{r:?}");

    }
}