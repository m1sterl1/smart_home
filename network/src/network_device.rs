use std::net::ToSocketAddrs;
use std::sync::{Arc, RwLock};

use crate::{
    device::Device,
    transport::{SharedDevice, Transport},
    Result,
};

pub struct NetworkDevice<L: Transport> {
    listener: L,
    device: SharedDevice,
}

impl<L: Transport> NetworkDevice<L> {
    pub fn new<A: ToSocketAddrs, D: Device + Send + Sync + 'static>(
        device: D,
        addr: A,
    ) -> Result<Self> {
        let listener = L::new(addr)?;
        let device = Arc::new(RwLock::new(device)) as SharedDevice;
        Ok(Self { listener, device })
    }

    pub fn listen(&self) -> Result<()> {
        self.listener.listen(self.device.clone())
    }
}
