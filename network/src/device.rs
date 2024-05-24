/// Provides Device trait, which makes devices capable to handle
/// CommandReqeuest
/// Device is implemented by devices from smart home

use crate::command::{CommandRequest, CommandResponse, RequestType, ResponseType};

use smart_home::devices::{Socket, Thermometer};

pub trait Device {
    fn process(&mut self, request: CommandRequest) -> CommandResponse;
}

impl Device for Socket {
    fn process(&mut self, request: CommandRequest) -> CommandResponse {
        let response_type = if self.id() != request.id() {
            ResponseType::Err("Id is not matched".into())
        } else {
            match request.req_type() {
                RequestType::SocketGetState => ResponseType::Success(self.to_string()),
                RequestType::SocketTurnOff => match self.turn_off() {
                    Ok(_) => ResponseType::Success("".into()),
                    Err(e) => ResponseType::Err(e.to_string()),
                },
                RequestType::SocketTurnOn => match self.turn_on() {
                    Ok(_) => ResponseType::Success("".into()),
                    Err(e) => ResponseType::Err(e.to_string()),
                },
                _ => ResponseType::Err("Wrong request".into()),
            }
        };
        CommandResponse::new(self.id(), response_type)
    }
}

impl Device for Thermometer {
    fn process(&mut self, request: CommandRequest) -> CommandResponse {
        let response_type = if self.id() != request.id() {
            ResponseType::Err("Requested is not matched".into())
        } else {
            match request.req_type() {
                RequestType::ThermGetTemp => match self.get_temperature() {
                    Ok(t) => ResponseType::Success(t.to_string()),
                    Err(e) => ResponseType::Err(e.to_string()),
                },
                _ => ResponseType::Err("Wrong request".into()),
            }
        };
        CommandResponse::new(self.id(), response_type)
    }
}
