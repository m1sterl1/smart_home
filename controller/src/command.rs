/// Commands and Responses for SocketClient and SocketServer
/// serialized and deserialized with serde_json
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, String>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    SocketTurnOn { id: String },
    SocketTurnOff { id: String },
    SocketGetState { id: String },
    ThermGetAddress { id: String }, // Get thermometer udp socket address
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CommandResponse {
    SocketTurnOn(Result<()>),
    SocketTurnOff(Result<()>),
    SocketGetState(Result<String>),
    ThermGetAddress(Result<String>),
}
