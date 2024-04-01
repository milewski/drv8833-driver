use crate::bridge::PwmBridge;
use crate::driver::Driver;

pub struct PwmSplitDriver<IN1, IN2, IN3, IN4> {
    pub a: PwmBridge<IN1, IN2>,
    pub b: PwmBridge<IN3, IN4>,
}

impl<IN1, IN2, IN3, IN4> PwmSplitDriver<IN1, IN2, IN3, IN4> {
    pub fn new(in1: IN1, in2: IN2, in3: IN3, in4: IN4) -> Self {
        Self {
            a: PwmBridge::new(in1, in2, 0),
            b: PwmBridge::new(in3, in4, 0),
        }
    }
}

impl<IN1, IN2, IN3, IN4> Driver for PwmSplitDriver<IN1, IN2, IN3, IN4> {}

impl<IN1, IN2, IN3, IN4> PwmSplitDriver<IN1, IN2, IN3, IN4> {
    pub fn set_min_duty(&mut self, duty: u16) {
        self.a.set_min_duty(duty);
        self.b.set_min_duty(duty);
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
        let mut in1 = PwmPin::new(&[
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(50),
            PwmPinTransaction::set_duty_cycle(0),
        ]);

        let mut in2 = PwmPin::new(&[PwmPinTransaction::set_duty_cycle(0), PwmPinTransaction::set_duty_cycle(0)]);

        let mut in3 = PwmPin::new(&[
            PwmPinTransaction::set_duty_cycle(0),
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(100),
        ]);

        let mut in4 = PwmPin::new(&[
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(10),
            PwmPinTransaction::max_duty_cycle(100),
            PwmPinTransaction::set_duty_cycle(100),
        ]);

        let mut sleep = Pin::new(&[Transaction::set(High), Transaction::set(Low)]);

        let mut motor = MotorDriver::new_pwm_split(
            in1.clone(), in2.clone(), in3.clone(), in4.clone(), Some(sleep.clone()), None::<Pin>,
        );

        motor.wakeup()?;

        motor.a.forward(50)?;
        motor.b.reverse(10)?;

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
}
