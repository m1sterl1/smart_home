use std::fmt::Display;
use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpServer, Responder};
use smart_home::{DeviceInfoProvider, SmartHome, SmartHomeError};

type SmartHomeWeb = web::Data<Mutex<SmartHome>>;
type Result<T> = std::result::Result<T, SmartHomeError>;

#[get("/rooms")]
async fn rooms(smart_home: SmartHomeWeb) -> impl Responder {
    let rooms: Vec<String> = smart_home
        .lock()
        .unwrap()
        .get_rooms()
        .iter()
        .map(|r| r.to_string())
        .collect();
    let rooms: Result<Vec<String>> = Ok(rooms);
    serde_json::to_string(&rooms).unwrap()
}

#[get("/devices/{room}")]
async fn devices(smart_home: SmartHomeWeb, room: web::Path<String>) -> impl Responder {
    let response: Result<Vec<String>> = smart_home
        .lock()
        .unwrap()
        .devices(&room)
        .map(|d| d.iter().map(|r| r.to_string()).collect());
    serde_json::to_string(&response).unwrap()
}

#[post("/rooms/add/{room}")]
async fn rooms_add(smart_home: SmartHomeWeb, room: web::Path<String>) -> impl Responder {
    let response = smart_home.lock().unwrap().add_room(&room);
    serde_json::to_string(&response).unwrap()
}

#[post("/rooms/del/{room}")]
async fn rooms_del(smart_home: SmartHomeWeb, room: web::Path<String>) -> impl Responder {
    let response = smart_home.lock().unwrap().remove_room(&room);
    serde_json::to_string(&response).unwrap()
}

#[post("/devices/add/{room}/{device}")]
async fn devices_add(
    smart_home: SmartHomeWeb,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let (room, device) = params.into_inner();
    let response = smart_home.lock().unwrap().add_device(&room, &device);
    serde_json::to_string(&response).unwrap()
}

#[post("/devices/del/{room}/{device}")]
async fn devices_del(
    smart_home: SmartHomeWeb,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let (room, device) = params.into_inner();
    let response = smart_home.lock().unwrap().remove_device(&room, &device);
    serde_json::to_string(&response).unwrap()
}

type InfoProvider = web::Data<DeviceSource<Box<dyn Display>>>;

#[get("/report")]
async fn report(smart_home: SmartHomeWeb, info_provider: InfoProvider) -> impl Responder {
    let info_provider = &**info_provider;
    let response = smart_home.lock().unwrap().create_report(info_provider);
    serde_json::to_string(&response).unwrap()
}

use smart_home::{
    devices::{Socket, Thermometer},
    sources::DeviceSource,
};

fn create_provider() -> impl DeviceInfoProvider {
    // Device initialization
    let mut socket = Socket::new("Smart Socket v1.0");
    socket.turn_on().unwrap();
    let thermo = Thermometer::new("Thermometer v1.0");

    let mut info_provider = DeviceSource::new();
    info_provider
        .add_device("Socket", "Guestroom", Box::new(socket) as Box<dyn Display>)
        .unwrap();
    info_provider
        .add_device(
            "Thermometer",
            "Bathroom",
            Box::new(thermo) as Box<dyn Display>,
        )
        .unwrap();
    info_provider
}

pub async fn server_run(addr: &str) -> std::io::Result<()> {
    let smart_home = web::Data::new(Mutex::new(SmartHome::new("Web test")));
    HttpServer::new(move || {
        let info_provider = web::Data::new(create_provider());
        App::new()
            .app_data(smart_home.clone())
            .app_data(info_provider)
            .service(rooms)
            .service(rooms_add)
            .service(rooms_del)
            .service(devices)
            .service(devices_add)
            .service(devices_del)
            .service(report)
    })
    .bind(addr)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use std::{error::Error, time::Duration};

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    #[actix_web::test]
    async fn test_run() -> Result<()> {
        actix_web::rt::spawn(server_run("127.0.0.1:8080"));
        actix_web::rt::time::sleep(Duration::from_secs(1)).await;
        let client = Client::new();

        let resp = client
            .get("http://127.0.0.1:8080/rooms")
            .send()
            .await?
            .text()
            .await?;

        println!("{resp}");
        Ok(())
    }
}
