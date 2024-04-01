use std::sync::{Arc, Mutex};

use embedded_hal::digital::OutputPin;
use embedded_hal::pwm::SetDutyCycle;

use crate::bridge::remap;
use crate::driver::{Breaks, Driver, MotorDriverError, Movement, PwmMovement};
use crate::split_driver::SplitDriver;

pub struct PwmParallelDriver<IN1, IN2, IN3, IN4, PWM>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pwm: PWM,
    split: SplitDriver<IN1, IN2, IN3, IN4>,
    min_duty: u16,
}

impl<IN1, IN2, IN3, IN4, PWM> PwmParallelDriver<IN1, IN2, IN3, IN4, PWM>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pub fn new(in1: IN1, in2: IN2, in3: IN3, in4: IN4, pwm: PWM) -> Self {
        Self {
            pwm,
            min_duty: 0,
            split: SplitDriver::new(in1, in2, in3, in4),
        }
    }
}

impl<IN1, IN2, IN3, IN4, PWM> Driver for PwmParallelDriver<IN1, IN2, IN3, IN4, Arc<Mutex<PWM>>>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{}

impl<IN1, IN2, IN3, IN4, PWM> PwmParallelDriver<IN1, IN2, IN3, IN4, Arc<Mutex<PWM>>>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        PWM: SetDutyCycle,
{
    pub fn set_min_duty(&mut self, duty: u16) {
        self.min_duty = duty;
    }

    fn set_duty_cycle_percent(&self, percent: u8) -> Result<(), MotorDriverError> {
        if percent > 100 {
            return Err(MotorDriverError::InvalidRange);
        }

        let mut pwm = self.pwm.lock().map_err(|_| MotorDriverError::PwmLocked)?;

        let result = match percent {
            0 => pwm.set_duty_cycle_fully_off(),
            100 => pwm.set_duty_cycle_fully_on(),
            _ => {
                let remapped = remap(percent, self.min_duty, pwm.max_duty_cycle());

                pwm.set_duty_cycle(remapped)
            }
        };

        result.map_err(|_| MotorDriverError::UnableToSetDuty)?;

        Ok(())
    }
}

impl<IN1, IN2, IN3, IN4, PWM> PwmMovement for PwmParallelDriver<IN1, IN2, IN3, IN4, Arc<Mutex<PWM>>>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        PWM: SetDutyCycle,
{
    fn forward(&mut self, percent: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle_percent(percent)?;

        self.split.a.forward()?;
        self.split.b.forward()?;

        Ok(())
    }

    fn reverse(&mut self, percent: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle_percent(percent)?;

        self.split.a.reverse()?;
        self.split.b.reverse()?;

        Ok(())
    }
}

impl<IN1, IN2, IN3, IN4, PWM> Breaks for PwmParallelDriver<IN1, IN2, IN3, IN4, Arc<Mutex<PWM>>>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        PWM: SetDutyCycle,
{
    fn coast(&mut self) -> Result<(), MotorDriverError> {
        self.set_duty_cycle_percent(0)?;

        self.split.a.coast()?;
        self.split.b.coast()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorDriverError> {
        self.set_duty_cycle_percent(100)?;

        self.split.a.stop()?;
        self.split.b.stop()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::pin::Mock as Pin;
    use embedded_hal_mock::eh1::pin::State::{High, Low};
    use embedded_hal_mock::eh1::pin::Transaction;
    use embedded_hal_mock::eh1::pwm::Mock as PwmPin;
    use embedded_hal_mock::eh1::pwm::Transaction as PwmPinTransaction;

    use crate::driver::{Breaks, MotorDriver, MotorDriverError, PwmMovement};

    #[test]
    fn test_it_can_drive_each_bridge_independently() -> Result<(), MotorDriverError> {
        let mut in1 = Pin::new(&[
            Transaction::set(High),
            Transaction::set(Low),
            Transaction::set(Low),
            Transaction::set(High),
        ]);

        let mut in2 = Pin::new(&[
            Transaction::set(Low),
            Transaction::set(Low),
            Transaction::set(High),
            Transaction::set(High),
        ]);

        let mut in3 = Pin::new(&[
            Transaction::set(High),
            Transaction::set(Low),
            Transaction::set(Low),
            Transaction::set(High),
        ]);

        let mut in4 = Pin::new(&[
            Transaction::set(Low),
            Transaction::set(Low),
            Transaction::set(High),
            Transaction::set(High),
        ]);

        let mut pwm = PwmPin::new(&[
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(50),
            PwmPinTransaction::set_duty_cycle(0),
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(10),
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(100),
        ]);

        let mut motor = MotorDriver::new_pwm_parallel(
            in1.clone(), in2.clone(), in3.clone(), in4.clone(), pwm.clone(), None::<Pin>,
        );

        motor.forward(50)?;
        motor.coast()?;
        motor.reverse(10)?;
        motor.stop()?;

        in1.done();
        in2.done();
        in3.done();
        in4.done();

        pwm.done();

        Ok(())
    }
}
