use std::{error::Error, fmt::Display};

use super::utils::RandomValue;

type Result<T> = std::result::Result<T, ThermometerError>;

pub struct Thermometer {
    id: String,
    state: ThermometerState, // state
}

impl Thermometer {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            state: ThermometerState::Off,
        }
    }
    /// id getter
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn turn_on(&mut self) -> Result<()> {
        self.state = ThermometerState::On;
        Ok(())
    }
    pub fn turn_off(&mut self) -> Result<()> {
        self.state = ThermometerState::Off;
        Ok(())
    }
    pub fn get_temperature(&mut self) -> Result<f32> {
        let t = Self::choose();
        Ok(t)
    }
}

// Text representation used in report
impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl RandomValue for Thermometer {
    type Value = f32;
    const LOW: Self::Value = 20.;
    const MAX: Self::Value = 25.;
}

pub enum ThermometerState {
    On,
    Off,
}

impl Display for ThermometerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            ThermometerState::On => "State: on",
            ThermometerState::Off => "State: off",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug)]
pub enum ThermometerError {}

impl Display for ThermometerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thermometer device error")
    }
}

impl Error for ThermometerError {}

#[cfg(test)]
mod tests {
    use super::Thermometer;

    #[test]
    fn test_display() {
        let t = Thermometer::new("therm_123");
        assert_eq!(t.to_string(), "State: off".to_string());
    }
}
