use embedded_hal::digital::OutputPin;
use std::fmt::Debug;
use crate::driver::{MotorDriver, MotorDriverError};

pub struct Bridge<IN1: OutputPin, IN2: OutputPin> {
    in1: IN1,
    in2: IN2,
}

impl<IN1, IN2, EspError> MotorDriver<EspError> for Bridge<IN1, IN2>
    where
        IN1: OutputPin<Error=EspError>,
        IN2: OutputPin<Error=EspError>,
        EspError: Debug,
{
    fn forward(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.in1.set_high().map_err(|error| MotorDriverError::EspError(error))?;
        self.in2.set_low().map_err(|error| MotorDriverError::EspError(error))?;

        Ok(())
    }

    fn reverse(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.in1.set_low().map_err(|error| MotorDriverError::EspError(error))?;
        self.in2.set_high().map_err(|error| MotorDriverError::EspError(error))?;

        Ok(())
    }

    fn coast(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.in1.set_low().map_err(|error| MotorDriverError::EspError(error))?;
        self.in2.set_low().map_err(|error| MotorDriverError::EspError(error))?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.in1.set_high().map_err(|error| MotorDriverError::EspError(error))?;
        self.in2.set_high().map_err(|error| MotorDriverError::EspError(error))?;

        Ok(())
    }
}

impl<IN1: OutputPin, IN2: OutputPin> Bridge<IN1, IN2> {
    pub fn new(in1: IN1, in2: IN2) -> Self {
        Self { in1, in2 }
    }
}
