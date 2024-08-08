/// Implements NetworkDevice structure,
/// which wraps device from smart_home crate
/// and different kind of transports
use std::net::ToSocketAddrs;
use std::sync::{Arc, RwLock};

use crate::{
    device::Device,
    sync::{SharedDevice, Server},
    Result,
};

pub struct NetworkDevice<T: Server> {
    transport: T,
    device: SharedDevice,
}

impl<T: Server> NetworkDevice<T> {
    pub fn new<A: ToSocketAddrs, D: Device + Send + Sync + 'static>(
        device: D,
        addr: A,
    ) -> Result<Self> {
        let listener = T::new(addr)?;
        let device = Arc::new(RwLock::new(device)) as SharedDevice;
        Ok(Self {
            transport: listener,
            device,
        })
    }

    pub fn listen(&self) -> Result<()> {
        self.transport.listen(self.device.clone())
    }
}
