use std::{cell::RefCell, collections::HashMap, error::Error, net::ToSocketAddrs};

use smart_home::devices::Socket;
use stp::server::StpServer;

use crate::command::{Command, CommandResponse};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Server (Controller) for sockets, single threaded
/// contains socket devices
pub struct Controller {
    server: StpServer,
    // Socket storage <id, socket_device>
    storage: RefCell<HashMap<String, Socket>>,
}

impl Controller {
    /// Creates new instance with internal StpServer
    /// which binds to address `a`
    pub fn new<A: ToSocketAddrs>(a: A) -> Result<Self> {
        let server = StpServer::bind(a)?;
        let storage = RefCell::new(HashMap::default());
        Ok(Self { server, storage })
    }

    /// Creates new socket device with `id`
    /// and add it to server storage
    pub fn add_socket(&self, id: &str) {
        self.storage
            .borrow_mut()
            .insert(id.to_string(), Socket::new(id));
    }

    /// Listen to incoming connections (single theaded)
    /// If there is a connection, process it and send response back
    pub fn listen(&mut self) -> Result<()> {
        while let Some(Ok(mut conn)) = self.server.incoming().next() {
            println!("Got connection from {:?}", conn.peer_addr());
            while let Ok(command) = conn.recv_request() {
                let command = serde_json::de::from_str::<Command>(&command)?;
                println!("\tcommand {:?}", command);
                let response = self.process_command(command);
                let response = serde_json::ser::to_string(&response)?;
                conn.send_response(response)?;
            }
        }
        Ok(())
    }

    /// Process command depends on command type
    /// Send appropriate response back
    fn process_command(&self, command: Command) -> CommandResponse {
        match command {
            Command::TurnOn { id } => self.process_turn_on(&id),
            Command::TurnOff { id } => self.process_turn_off(&id),
            Command::GetState { id } => self.process_get_state(&id),
        }
    }

    // Process Command::TrunOn
    fn process_turn_on(&self, id: &str) -> CommandResponse {
        let result = self.turn_on(id);
        CommandResponse::TurnOn(result.map_err(|e| e.to_string()))
    }

    fn turn_on(&self, id: &str) -> Result<()> {
        let mut storage = self.storage.borrow_mut();
        let socket = storage.get_mut(id).ok_or("No socket")?;
        Ok(socket.turn_on()?)
    }

    /// Process Command::TrunOff
    fn process_turn_off(&self, id: &str) -> CommandResponse {
        let result = self.turn_off(id);
        CommandResponse::TurnOff(result.map_err(|e| e.to_string()))
    }

    fn turn_off(&self, id: &str) -> Result<()> {
        let mut storage = self.storage.borrow_mut();
        let socket = storage.get_mut(id).ok_or("No socket")?;
        Ok(socket.turn_off()?)
    }

    /// Process Command::GetState
    fn process_get_state(&self, id: &str) -> CommandResponse {
        let result = self.get_state(id);
        CommandResponse::GetState(result.map_err(|e| e.to_string()))
    }

    fn get_state(&self, id: &str) -> Result<String> {
        let mut storage = self.storage.borrow_mut();
        let socket = storage.get_mut(id).ok_or("No socket")?;
        Ok(socket.to_string())
    }
}
