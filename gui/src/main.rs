use std::process::exit;
use std::sync::{Arc, Mutex};

use iced::alignment::Vertical::Bottom;
use iced::widget::{button, column, row, text, text_input, Column, Row};
use iced::Alignment::Center;
use iced::{time, Size, Subscription, Task};

use network::command::CommandRequest;
use network::sync::{Client, NetworkDevice, TCPClient, TCPServer};
use smart_home::devices::Socket;

const CONNECTION_STRING: &str = "127.0.0.1:8080";
const TEXT_INPUT_SIZE: f32 = 250.0;
const SOCKET_ID: &str = "id";

pub fn main() -> iced::Result {
    // Starting network device for test purposes
    spawn_server().unwrap_or_else(|e| {
        println!("Error spawning server \n{e}");
        exit(1)
    });
    iced::application("Net Socket", SocketUI::update, SocketUI::view)
        .subscription(SocketUI::subscribtion)
        .window_size(Size::new(500.0, 200.0))
        .run_with(|| (SocketUI::new(), Task::none()))
}

fn spawn_server() -> network::errors::Result<()> {
    let socket1 = Socket::new(SOCKET_ID);
    let socket1_tcp: NetworkDevice<TCPServer> = NetworkDevice::new(socket1, CONNECTION_STRING)?;
    std::thread::spawn(move || socket1_tcp.listen());
    Ok(())
}

fn parse_response(response: &str) -> Option<&str> {
    response.rsplit_once(" ").map(|(_, p)| p)
}

type SocketClient = Arc<Mutex<TCPClient>>;

#[derive(Default)]
struct SocketUI {
    client: Option<SocketClient>,
    state: SocketState,
    power: String,
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
    Power(String),
    InputChanged(String),
}

impl SocketUI {
    fn new() -> Self {
        Self {
            client: None,
            state: SocketState::Disconnected,
            power: "0W".into(),
            status: "Disconnected".into(),
            connection_string: CONNECTION_STRING.into(),
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Connect(a) => match TCPClient::new(a) {
                Ok(client) => {
                    self.client = Some(Arc::new(Mutex::new(client)));
                    self.state = SocketState::Off;
                    self.status = "Connected".into();
                }
                Err(e) => {
                    self.status = e.to_string();
                }
            },
            Message::TurnOn => {
                if let Some(client) = self.client.as_ref() {
                    match client
                        .lock()
                        .unwrap()
                        .get(CommandRequest::builder().socket(SOCKET_ID).turn_on())
                    {
                        Ok(_) => {
                            self.state = SocketState::On;
                            self.status = "Status: On".into();
                        }
                        Err(e) => {
                            self.status = e.to_string();
                        }
                    }
                };
            }
            Message::TurnOff => {
                if let Some(client) = self.client.as_ref() {
                    match client
                        .lock()
                        .unwrap()
                        .get(CommandRequest::builder().socket(SOCKET_ID).turn_off())
                    {
                        Ok(_) => {
                            self.state = SocketState::Off;
                            self.power = "0W".into();
                            self.status = "Status: Off".into();
                        }
                        Err(e) => self.status = e.to_string(),
                    }
                };
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
            self.connection_view().padding(30),
            // control row
            self.control_view(),
            // status
            text(&self.status).align_y(Bottom)
        ]
        .align_x(Center)
    }

    fn subscribtion(&self) -> Subscription<Message> {
        match self.state {
            SocketState::Disconnected => Subscription::none(),
            SocketState::Off => Subscription::none(),
            SocketState::On => {
                let client = self.client.clone().unwrap();
                time::every(time::Duration::from_secs(1)).map(move |_| {
                    let resp = client
                        .lock()
                        .unwrap()
                        .get(CommandRequest::builder().socket(SOCKET_ID).get_state());
                    if let Ok(resp) = resp {
                        if let Some(resp) = resp.success() {
                            if let Some(power) = parse_response(&resp) {
                                return Message::Power(power.into());
                            }
                        }
                    }
                    Message::Power("0W".into())
                })
            }
        }
    }

    fn connection_view(&self) -> Row<Message> {
        // text input
        let text_input = text_input(CONNECTION_STRING, &self.connection_string)
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

    fn control_view(&self) -> Column<Message> {
        match self.state {
            SocketState::Disconnected => Column::new(),
            SocketState::On => {
                column![
                    button("Turn Off").on_press(Message::TurnOff),
                    text(self.power.to_string()).size(20)
                ]
            }
            SocketState::Off => {
                column![
                    button("Turn On").on_press(Message::TurnOn),
                    text(self.power.to_string()).size(20)
                ]
            }
        }
    }
}
