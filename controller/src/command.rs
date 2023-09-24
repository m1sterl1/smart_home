/// Commands and Responses for SocketClient and SocketServer
/// serialized and deserialized with serde_json
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, String>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    TurnOn { id: String },
    TurnOff { id: String },
    GetState { id: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CommandResponse {
    TurnOn(Result<()>),
    TurnOff(Result<()>),
    GetState(Result<String>),
}
