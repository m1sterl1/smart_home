use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    net::ToSocketAddrs,
    net::UdpSocket, 
    sync::{Arc, Mutex}, thread::spawn
};

use smart_home::devices::{Socket, Thermometer};
use stp::{server::{StpServer, StpConnection}, client::StpClient};

use crate::command::{Command, CommandResponse};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

const UDP_START_PORT: u32 = 20000;

/// Server (Controller) for sockets, single threaded
/// contains socket devices
pub struct Controller {
    server: StpServer,
    // Socket storage <id, socket_device>
    socket_storage: Mutex<HashMap<String, Socket>>,
    // Thermometer storage
    therm_storage: Mutex<HashMap<String, Thermometer>>
}

impl Controller {
    /// Creates new instance with internal StpServer
    /// which binds to address `a`
    pub fn new<A: ToSocketAddrs>(a: A) -> Result<Arc<Self>> {
        let server = StpServer::bind(a)?;
        let socket_storage = Mutex::new(HashMap::default());
        let therm_storage = Mutex::new(HashMap::default());
        Ok(Arc::new(Self { server, socket_storage, therm_storage }))
    }

    /// Creates new socket device with `id`
    /// and add it to server storage, overwrite if exists
    pub fn add_socket(self: &Arc<Self>, id: &str) {
        self.socket_storage
            .lock()
            .unwrap()
            .insert(id.to_string(), Socket::new(id));
    }

    /// Creates new thermometer device with `id`
    /// and add it to server storage, overwrite if exists
    pub fn add_thermometer(self: &Arc<Self>, id: &str) {
        self.therm_storage
            .lock()
            .unwrap()
            .insert(id.to_string(), Thermometer::new(id));
    }

    /// Listen to incoming connections
    /// If there is a connection, process it and send response back
    pub fn listen(self: &Arc<Self>) {
        while let Some(Ok(conn)) = self.server.incoming().next() {
            let this = self.clone();
            spawn(||this.handle(conn));
        }
    }

    /// Handle new connection
    fn handle(self: Arc<Self>, mut conn: StpConnection) -> Result<()>{
        println!("Got connection from {:?}", conn.peer_addr());
        while let Ok(command) = conn.recv_request() {
            let command = serde_json::de::from_str::<Command>(&command)?;
            println!("\tcommand {:?}", command);
            let response = self.process_command(command);
            let response = serde_json::ser::to_string(&response)?;
            conn.send_response(response)?;
        }
        Ok(())
    }

    /// Process command depends on command type
    /// Send appropriate response back
    fn process_command(self: &Arc<Self>, command: Command) -> CommandResponse {
        match command {
            Command::SocketTurnOn { id } => self.process_s_turn_on(&id),
            Command::SocketTurnOff { id } => self.process_s_turn_off(&id),
            Command::SocketGetState { id } => self.process_s_get_state(&id),
            // Command::ThermGetAddress { id } => self.process_t_get_address(id),
            Command::ThermGetAddress { id } => todo!(),
        }
    }

    // Process Command::TrunOn
    fn process_s_turn_on(self: &Arc<Self>, id: &str) -> CommandResponse {
        let result = self.s_turn_on(id);
        CommandResponse::SocketTurnOn(result.map_err(|e| e.to_string()))
    }

    fn s_turn_on(self: &Arc<Self>, id: &str) -> Result<()> {
        let mut storage = self.socket_storage.lock().unwrap();
        let socket = storage.get_mut(id).ok_or("No socket")?;
        Ok(socket.turn_on()?)
    }

    /// Process Command::TrunOff
    fn process_s_turn_off(self: &Arc<Self>, id: &str) -> CommandResponse {
        let result = self.s_turn_off(id);
        CommandResponse::SocketTurnOff(result.map_err(|e| e.to_string()))
    }

    fn s_turn_off(self: &Arc<Self>, id: &str) -> Result<()> {
        let mut storage = self.socket_storage.lock().unwrap();
        let socket = storage.get_mut(id).ok_or("No socket")?;
        Ok(socket.turn_off()?)
    }

    /// Process Command::GetState
    fn process_s_get_state(self: &Arc<Self>, id: &str) -> CommandResponse {
        let result = self.s_get_state(id);
        CommandResponse::SocketGetState(result.map_err(|e| e.to_string()))
    }

    fn s_get_state(self: &Arc<Self>, id: &str) -> Result<String> {
        let mut storage = self.socket_storage.lock().unwrap();
        let socket = storage.get_mut(id).ok_or("No socket")?;
        Ok(socket.to_string())
    }

    // / Emulate getting address from thermometer device
    // / Get free udp port from pull, start listen to port in separate thead
    // fn process_t_get_address(&self, id: &str) -> CommandResponse{

    // }

    // fn t_get_address(&self, id: &str) -> Result<String> {
    //     let mut storage = self.therm_storage.borrow_mut();
    //     let therm = storage.get_mut(id).ok_or("No thermometer")?;
    //     for i in 0..100 {
    //         let port = UDP_START_PORT + i;
    //         if let Ok(udp_socket) = UdpSocket::bind(format!("0.0.0.0:{i}")){
                
    //         }
    //     }
    //     Ok("hello".to_string())
    // }

    // fn thermometer_listen(udp_socket: UdpSocket){
        
    // }

}
