use embedded_hal::digital::OutputPin;

use crate::bridge::Bridge;
use crate::driver::Driver;

pub struct SplitDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pub a: Bridge<IN1, IN2>,
    pub b: Bridge<IN3, IN4>,
}

impl<IN1: OutputPin, IN2: OutputPin, IN3: OutputPin, IN4: OutputPin> SplitDriver<IN1, IN2, IN3, IN4> {
    pub fn new(in1: IN1, in2: IN2, in3: IN3, in4: IN4) -> Self {
        Self {
            a: Bridge::new(in1, in2),
            b: Bridge::new(in3, in4),
        }
    }
}

impl<IN1, IN2, IN3, IN4> Driver for SplitDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{}

#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::pin::Mock as Pin;
    use embedded_hal_mock::eh1::pin::State::{High, Low};
    use embedded_hal_mock::eh1::pin::Transaction;
    use embedded_hal_mock::eh1::pwm::Mock as PwmPin;
    use embedded_hal_mock::eh1::pwm::Transaction as PwmPinTransaction;

    use crate::driver::{Breaks, MotorDriver, MotorDriverError, Movement};

    #[test]
    fn it_can_drive_each_bridge_independently() -> Result<(), MotorDriverError> {
        let mut in1 = Pin::new(&[Transaction::set(High), Transaction::set(Low)]);
        let mut in2 = Pin::new(&[Transaction::set(Low), Transaction::set(Low)]);
        let mut in3 = Pin::new(&[Transaction::set(Low), Transaction::set(High)]);
        let mut in4 = Pin::new(&[Transaction::set(High), Transaction::set(High)]);

        let mut sleep = Pin::new(&[Transaction::set(High), Transaction::set(Low)]);

        let mut motor = MotorDriver::new_split(
            in1.clone(), in2.clone(), in3.clone(), in4.clone(), Some(sleep.clone()), None::<Pin>,
        );

        motor.wakeup()?;

        motor.a.forward()?;
        motor.b.reverse()?;

        motor.a.coast()?;
        motor.b.stop()?;

        motor.sleep()?;

        in1.done();
        in2.done();
        in3.done();
        in4.done();

        sleep.done();

        Ok(())
    }

    #[test]
    fn it_can_be_driven_by_pwm() -> Result<(), MotorDriverError> {
        let mut in1 = Pin::new(&[Transaction::set(High), Transaction::set(Low)]);
        let mut in2 = Pin::new(&[Transaction::set(Low), Transaction::set(Low)]);
        let mut in3 = Pin::new(&[Transaction::set(Low), Transaction::set(High)]);
        let mut in4 = Pin::new(&[Transaction::set(High), Transaction::set(High)]);
        let mut pwm = PwmPin::new(&[
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(50),
        ]);

        let mut motor = MotorDriver::new_pwm_split_single(
            in1.clone(), in2.clone(), in3.clone(), in4.clone(), pwm.clone(), None::<Pin>,
        );

        motor.set_duty_cycle(50)?;

        motor.a.forward()?;
        motor.b.reverse()?;
        motor.a.coast()?;
        motor.b.stop()?;

        in1.done();
        in2.done();
        in3.done();
        in4.done();

        pwm.done();

        Ok(())
    }
}
