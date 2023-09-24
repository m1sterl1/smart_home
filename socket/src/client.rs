/// Tcp socket client
/// Turn socket on and off, current state and power consuption 

use std::{error::Error, net::ToSocketAddrs};

use stp::{client::StpClient, error::ConnectResult};

use crate::command::{Command, CommandResponse};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct SocketCLient{
    client: StpClient
}

/// Connects to SocketServer which contains devices
/// Send commands to devices by ids
impl SocketCLient{
    /// Creates new instance, connects internal StpClient to `addrs`
    pub fn new<A:ToSocketAddrs>(addrs: A) -> ConnectResult<Self>{
        let client = StpClient::connect(addrs)?;
        Ok(Self {client})
    }

    /// Turn on socket device with `id`
    pub fn turn_on(&mut self, id: &str) -> Result<CommandResponse>{
        let command = Command::TurnOn { id: id.to_string() };
        self.send_command(command)
    }

    /// Turn off socket device with `id`
    pub fn turn_off(&mut self, id: &str) -> Result<CommandResponse>{
        let command = Command::TurnOff { id: id.to_string() };
        self.send_command(command)
    }

    /// Get state of socket device with `id`
    pub fn get_state(&mut self, id: &str) -> Result<CommandResponse>{
        let command = Command::GetState { id: id.to_string() };
        self.send_command(command)
    }

    /// Send command to SocketServer
    fn send_command(&mut self, command: Command) -> Result<CommandResponse>{
        let command = serde_json::ser::to_string(&command)?;
        let response = self.client.send_request(command)?;
        let response = serde_json::de::from_str::<CommandResponse>(&response)?;
        Ok(response)
    }
}