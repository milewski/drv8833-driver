use embedded_hal::digital::OutputPin;

use crate::bridge::Bridge;
use crate::driver::{Breaks, Driver, MotorDriverError, Movement};

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

impl<IN1, IN2, IN3, IN4> Movement for ParallelDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    fn forward(&mut self) -> Result<(), MotorDriverError> {
        self.a.forward()?;
        self.b.forward()?;

        Ok(())
    }

    fn reverse(&mut self) -> Result<(), MotorDriverError> {
        self.a.reverse()?;
        self.b.reverse()?;

        Ok(())
    }
}

impl<IN1, IN2, IN3, IN4> Breaks for ParallelDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    fn coast(&mut self) -> Result<(), MotorDriverError> {
        self.a.coast()?;
        self.b.coast()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorDriverError> {
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

#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::pin::State::{High, Low};
    use embedded_hal_mock::eh1::pin::{Mock as Pin, Transaction};

    use crate::driver::{Breaks, MotorDriver, MotorDriverError, Movement};

    #[test]
    fn test_all_operation_are_driven_simultaneously() -> Result<(), MotorDriverError> {
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

        let mut sleep = Pin::new(&[Transaction::set(High), Transaction::set(Low)]);
        let mut motor = MotorDriver::new_parallel(
            in1.clone(), in2.clone(), in3.clone(), in4.clone(), Some(sleep.clone()), None::<Pin>,
        );

        motor.wakeup()?;
        motor.forward()?;
        motor.coast()?;
        motor.reverse()?;
        motor.stop()?;
        motor.sleep()?;

        in1.done();
        in2.done();
        in3.done();
        in4.done();

        sleep.done();

        Ok(())
    }
}
