use std::str::FromStr;

use reqwest::{Client, IntoUrl, Url};

use crate::Result;

pub struct SmartHomeClient{
    base: Url,
    client: Client,
}

impl SmartHomeClient{
    pub fn new<U:IntoUrl>(url: U) -> Result<Self>{
        Ok(Self{base: url.into_url()?, client:Client::new()})
    }

    /// Get smarthome rooms
    pub async fn rooms(&self) -> Result<String>{
        let s = self.client
        .get(self.base.join("/rooms")?)
        .send().await?
        .text().await?;
        Ok(s)
    }

    /// Add room
    pub async fn rooms_add(&self, room: &str) -> Result<String>{
        let s = self.client
        .post(self.base.join(&format!("/rooms/add/{room}"))?)
        .send().await?
        .text().await?;
        Ok(s)
    }

    /// Remove room
    pub async fn rooms_del(&self, room: &str) -> Result<String>{
        let s = self.client
        .post(self.base.join(&format!("/rooms/del/{room}"))?)
        .send().await?
        .text().await?;
        Ok(s)
    }

    /// Devices
    pub async fn devices(&self, room: &str) -> Result<String>{
        let s = self.client
        .get(self.base.join(&format!("/devices/{room}"))?)
        .send().await?
        .text().await?;
        Ok(s)
    }

    /// Add device
    pub async fn devices_add(&self, room: &str, device: &str) -> Result<String>{
        let s = self.client
        .post(self.base.join(&format!("/devices/add/{room}/{device}"))?)
        .send().await?
        .text().await?;
        Ok(s)
    }

    /// Add device
    pub async fn devices_del(&self, room: &str, device: &str) -> Result<String>{
        let s = self.client
        .post(self.base.join(&format!("/devices/del/{room}/{device}"))?)
        .send().await?
        .text().await?;
        Ok(s)
    }

    /// Report
    pub async fn report(&self) -> Result<String>{
        let s = self.client
        .get(self.base.join("/report")?)
        .send().await?
        .text().await?;
        Ok(s)
    }
}