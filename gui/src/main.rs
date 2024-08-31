use iced::widget::{button, column, row, text, text_input, Column, Row};

use network::command::CommandRequest;
use network::sync::{Client, TCPClient};

const CONNECTION_PLACEHODER: &str = "http://127.0.0.1:8080";
const TEXT_INPUT_SIZE: f32 = 250.0;
const BUTTON_SIZE: f32 = 100.0;
const SOCKET_ID: &str = "id";

pub fn main() -> iced::Result {
    iced::application("Net Socket", SocketUI::update, SocketUI::view).run()
}

#[derive(Default)]
struct SocketUI {
    client: Option<TCPClient>,
    state: SocketState,
    power: u32,
    status: String,
    connection_string: String,
}

#[derive(Default)]
enum SocketState {
    #[default]
    Disconnected,
    On,
    Off,
}

#[derive(Clone, Debug)]
enum Message {
    Connect(String),
    TurnOn,
    TurnOff,
    Power(u32),
    InputChanged(String),
}

impl SocketUI {
    fn update(&mut self, message: Message) {
        match message {
            Message::Connect(a) => match TCPClient::new(a) {
                Ok(client) => {
                    self.client = Some(client);
                    self.state = SocketState::On;
                }
                Err(e) => {
                    self.status = e.to_string();
                }
            },
            Message::TurnOn => {
                self.client.as_mut().map(|client| {
                    match client.send(CommandRequest::builder().socket(SOCKET_ID).turn_on()) {
                        Ok(_) => self.state = SocketState::On,
                        Err(e) => self.status = e.to_string(),
                    }
                });
            }
            Message::TurnOff => {
                self.client.as_mut().map(|client| {
                    match client.send(CommandRequest::builder().socket(SOCKET_ID).turn_off()) {
                        Ok(_) => {
                            self.state = SocketState::Off;
                            self.power = 0;
                        }
                        Err(e) => self.status = e.to_string(),
                    }
                });
            }
            Message::Power(p) => {
                self.power = p;
            }
            Message::InputChanged(s) => {
                self.connection_string = s;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            // connection row
            self.connection_row(),
            // control row
            // status
            text(&self.status)
        ]
    }

    fn connection_row(&self) -> Row<Message> {
        // text input
        let text_input = text_input(CONNECTION_PLACEHODER, &self.connection_string)
            .on_input(Message::InputChanged)
            .width(TEXT_INPUT_SIZE);
        // submit button
        let mut button = button("Connect");
        // disable button when socket already connected
        if let SocketState::Disconnected = self.state {
            button = button.on_press(Message::Connect(self.connection_string.clone()))
        };
        row![text_input, button]
    }
}
