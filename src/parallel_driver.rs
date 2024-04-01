use std::fmt::Debug;

use embedded_hal::digital::OutputPin;
use crate::bridge::Bridge;

use crate::driver::{Driver, MotorDriver, MotorDriverError};

pub struct ParallelDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    a: Bridge<IN1, IN2>,
    b: Bridge<IN3, IN4>,
}

impl<IN1, IN2, IN3, IN4> Driver for ParallelDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{}

impl<IN1, IN2, IN3, IN4, EspError> MotorDriver<EspError> for ParallelDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin<Error=EspError>,
        IN2: OutputPin<Error=EspError>,
        IN3: OutputPin<Error=EspError>,
        IN4: OutputPin<Error=EspError>,
        EspError: Debug,
{
    fn forward(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.a.forward()?;
        self.b.forward()?;

        Ok(())
    }

    fn reverse(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.a.reverse()?;
        self.b.reverse()?;

        Ok(())
    }

    fn coast(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.a.coast()?;
        self.b.coast()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.a.stop()?;
        self.b.stop()?;

        Ok(())
    }
}

impl<IN1, IN2, IN3, IN4> ParallelDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pub fn new(in1: IN1, in2: IN2, in3: IN3, in4: IN4) -> Self {
        Self {
            a: Bridge::new(in1, in2),
            b: Bridge::new(in3, in4),
        }
    }
}
