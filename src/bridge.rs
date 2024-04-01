use embedded_hal::digital::OutputPin;
use embedded_hal::pwm::SetDutyCycle;

use crate::driver::{Breaks, MotorDriverError, Movement, PwmMovement};

pub fn remap(value: u8, min: u16, max: u16) -> u16 {
    let percentage = value as f32 / 100.0;
    let min = min as f32;
    let max = max as f32;

    (percentage * (max - min) + min) as u16
}

pub struct PwmBridge<IN1, IN2> {
    bridge: Bridge<IN1, IN2>,
    min_duty: u16,
}

/// Holds the reference to each pin used to drive the motor forward or reverse.
pub struct Bridge<IN1, IN2> {
    in1: IN1,
    in2: IN2,
}

impl<IN1: SetDutyCycle, IN2: SetDutyCycle> PwmMovement for PwmBridge<IN1, IN2> {
    fn forward(&mut self, percent: u8) -> Result<(), MotorDriverError> {
        let percent = remap(percent, self.min_duty, self.bridge.in1.max_duty_cycle());

        self.bridge
            .in1
            .set_duty_cycle(percent)
            .map_err(|_| MotorDriverError::UnableToSetDuty)?;

        self.bridge
            .in2
            .set_duty_cycle_fully_off()
            .map_err(|_| MotorDriverError::UnableToSetDuty)?;

        Ok(())
    }

    fn reverse(&mut self, percent: u8) -> Result<(), MotorDriverError> {
        let percent = remap(percent, self.min_duty, self.bridge.in2.max_duty_cycle());

        self.bridge
            .in1
            .set_duty_cycle_fully_off()
            .map_err(|_| MotorDriverError::UnableToSetDuty)?;

        self.bridge
            .in2
            .set_duty_cycle(percent)
            .map_err(|_| MotorDriverError::UnableToSetDuty)?;

        Ok(())
    }
}

impl<IN1: SetDutyCycle, IN2: SetDutyCycle> Breaks for PwmBridge<IN1, IN2> {
    fn coast(&mut self) -> Result<(), MotorDriverError> {
        self.bridge
            .in1
            .set_duty_cycle_fully_off()
            .map_err(|_| MotorDriverError::GpioError)?;

        self.bridge
            .in2
            .set_duty_cycle_fully_off()
            .map_err(|_| MotorDriverError::GpioError)?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorDriverError> {
        self.bridge.in1.set_duty_cycle_fully_on().map_err(|_| MotorDriverError::GpioError)?;
        self.bridge.in2.set_duty_cycle_fully_on().map_err(|_| MotorDriverError::GpioError)?;

        Ok(())
    }
}

impl<IN1: OutputPin, IN2: OutputPin> Breaks for Bridge<IN1, IN2> {
    fn coast(&mut self) -> Result<(), MotorDriverError> {
        self.in1.set_low().map_err(|_| MotorDriverError::GpioError)?;
        self.in2.set_low().map_err(|_| MotorDriverError::GpioError)?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorDriverError> {
        self.in1.set_high().map_err(|_| MotorDriverError::GpioError)?;
        self.in2.set_high().map_err(|_| MotorDriverError::GpioError)?;

        Ok(())
    }
}

impl<IN1: OutputPin, IN2: OutputPin> Movement for Bridge<IN1, IN2> {
    fn forward(&mut self) -> Result<(), MotorDriverError> {
        self.in1.set_high().map_err(|_| MotorDriverError::GpioError)?;
        self.in2.set_low().map_err(|_| MotorDriverError::GpioError)?;

        Ok(())
    }

    fn reverse(&mut self) -> Result<(), MotorDriverError> {
        self.in1.set_low().map_err(|_| MotorDriverError::GpioError)?;
        self.in2.set_high().map_err(|_| MotorDriverError::GpioError)?;

        Ok(())
    }
}

impl<IN1, IN2> Bridge<IN1, IN2> {
    pub fn new(in1: IN1, in2: IN2) -> Self {
        Self { in1, in2 }
    }
}

impl<IN1, IN2> PwmBridge<IN1, IN2> {
    pub fn new(in1: IN1, in2: IN2, min_duty: u16) -> Self {
        Self {
            bridge: Bridge::new(in1, in2),
            min_duty,
        }
    }

    pub fn set_min_duty(&mut self, duty: u16) {
        self.min_duty = duty;
    }
}
