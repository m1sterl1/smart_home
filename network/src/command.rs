/// Commands and Responses for SocketClient and SocketServer
/// serialized and deserialized with serde_json
use crate::Result;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRequest{
    id: String,
    request: RequestType,
}

impl CommandRequest{

    pub fn request_from(buf: &[u8]) -> Result<CommandRequest>{
        let req:CommandRequest = serde_json::from_slice(buf)?;
        Ok(req)
    }

    /// id getter
    pub fn id(&self) -> &str{
        &self.id
    }
    pub fn req_type(&self) -> &RequestType{
        &self.request
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    SocketTurnOn,
    SocketTurnOff,
    SocketGetState,
    ThermGetTemp, // Get thermometer udp socket address
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommandResponse{
    id: String,
    response: ResponseType
}

impl CommandResponse{
    pub fn new(id: &str, response: ResponseType) -> Self{
        Self { id:id.to_string(), response }
    }
}

impl From<CommandResponse> for Vec<u8>{
    fn from(value: CommandResponse) -> Self {
        serde_json::to_vec(&value).unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ResponseType {
    Success(String),
    Err(String),
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
        CommandRequest{
            id:self.0.to_string(),
            request: RequestType::SocketTurnOn
        }
    }
    pub fn turn_off(self) -> CommandRequest{
        CommandRequest{
            id:self.0.to_string(),
            request: RequestType::SocketTurnOff
        }
    }
    pub fn get_state(self) -> CommandRequest{
        CommandRequest{
            id:self.0.to_string(),
            request: RequestType::SocketGetState
        }
    }
}

impl ThermRequestBuilder<'_>{
    pub fn get_temp(self) -> CommandRequest{
        CommandRequest{
            id:self.0.to_string(),
            request: RequestType::ThermGetTemp
        }    }
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