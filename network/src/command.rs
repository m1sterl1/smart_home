/// Commands and Responses for SocketClient and SocketServer
/// serialized and deserialized with serde_json
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, String>;

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandRequest {
    SocketTurnOn { id: String },
    SocketTurnOff { id: String },
    SocketGetState { id: String },
    ThermGetTemp { id: String }, // Get thermometer udp socket address
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CommandResponse {
    SocketTurnOn(Result<()>),
    SocketTurnOff(Result<()>),
    SocketGetState(Result<String>),
    ThermGetTemp(Result<String>),
}

pub struct CommandRequestBuilder;
pub struct SocketRequestBuilder<'a>(&'a str); // id
pub struct ThermRequestBuilder<'a>(&'a str); // id

impl CommandRequestBuilder{
    pub fn socket(self, id: &str)-> SocketRequestBuilder{  
        SocketRequestBuilder(id)
    }

    pub fn therm(self, id: &str)-> ThermRequestBuilder{  
        ThermRequestBuilder(id)
    }
}

impl SocketRequestBuilder<'_>{
    pub fn turn_on(self) -> CommandRequest{
        CommandRequest::SocketTurnOn { id: self.0.to_string() }
    }
    pub fn turn_off(self) -> CommandRequest{
        CommandRequest::SocketTurnOff { id: self.0.to_string() }
    }
    pub fn get_state(self) -> CommandRequest{
        CommandRequest::SocketGetState { id: self.0.to_string() }
    }
}

impl ThermRequestBuilder<'_>{
    pub fn get_temp(self) -> CommandRequest{
        CommandRequest::ThermGetTemp { id: self.0.to_string() }
    }
}

impl CommandRequest{
    pub fn new() -> CommandRequestBuilder{
        CommandRequestBuilder
    }
}


#[cfg(test)]
mod tests{
    use super::CommandRequest;

    #[test]
    fn command_request(){
        let request = CommandRequest::new()
        .socket("socket_123")
        .get_state();
    }
}