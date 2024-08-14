use std::fmt::Display;

use thiserror::Error;

use super::utils::RandomValue;

type Result<T> = std::result::Result<T, SocketError>;

pub struct Socket {
    id: String,         // description
    state: SocketState, // current state
}

impl Socket {
    pub fn new(desc: &str) -> Self {
        // Random basic value of power consumption
        Self {
            id: desc.to_string(),
            state: SocketState::Off,
        }
    }
    /// id getter
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Turn socket on
    pub fn turn_on(&mut self) -> Result<()> {
        self.state = SocketState::On;
        Ok(())
    }
    /// Turn socket off
    pub fn turn_off(&mut self) -> Result<()> {
        self.state = SocketState::Off;
        Ok(())
    }
    /// Returns current power consumption (emulation)
    pub fn power_consuption(&self) -> f32 {
        match self.state {
            SocketState::On => Socket::choose(),
            SocketState::Off => 0.0,
        }
    }
}

// Text representation used in report
impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, power consumption {:.1}W",
            self.state,
            self.power_consuption()
        )
    }
}

impl RandomValue for Socket {
    type Value = f32;
    const LOW: f32 = 20.;
    const MAX: f32 = 1000.;
}

pub enum SocketState {
    On,
    Off,
}

impl Display for SocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            SocketState::On => "State: on",
            SocketState::Off => "State: off",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Error)]
pub enum SocketError {
    // Different error types
    #[error("DeviceErrorState")]
    DeviceErrorState,
    #[error("Max voltage value 250V excceed")]
    MaxVoltageExceed,
}

#[cfg(test)]
mod tests {
    use super::Socket;

    #[test]
    fn test_display() {
        let t = Socket::new("Test");
        assert_eq!(
            t.to_string(),
            "State: off, power consumption 0.0W".to_string()
        );
    }
}
