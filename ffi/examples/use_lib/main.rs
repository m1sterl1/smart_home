mod libload;

use libload::{Result, SocketWrapper};

fn main() -> Result<()> {
    let socket = SocketWrapper::new("socketid")?;
    println!("Power consumption {}w", socket.power()?);
    println!("Turn on socket");
    socket.turn_on()?;
    println!("Power consumption {}w", socket.power()?);
    Ok(())
}
