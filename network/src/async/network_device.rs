/// Implements NetworkDevice structure,
/// which wraps device from smart_home crate
/// and different kind of transports
use std::sync::Arc;
use tokio::{net::ToSocketAddrs, sync::RwLock};

use crate::{device::Device, r#async::SharedDevice, Result};

use super::ServerAsync;

pub struct NetworkDeviceAsync<T: ServerAsync> {
    transport: T,
    device: SharedDevice,
}

impl<T: ServerAsync> NetworkDeviceAsync<T> {
    pub async fn new<A: ToSocketAddrs + Send, D: Device + Send + Sync + 'static>(
        device: D,
        addr: A,
    ) -> Result<Self> {
        let listener = T::new(addr).await?;
        let device = Arc::new(RwLock::new(device)) as SharedDevice;
        Ok(Self {
            transport: listener,
            device,
        })
    }

    pub async fn listen(&self) -> Result<()> {
        self.transport.listen(self.device.clone()).await
    }
}
