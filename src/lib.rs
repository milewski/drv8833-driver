use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use embedded_hal::digital::{Error, ErrorType, OutputPin};

mod parallel_driver;
mod sync_driver;
mod driver;
mod bridge;

#[cfg(test)]
mod tests {
    use embedded_hal::digital::{ErrorKind, PinState};
    use crate::driver::{DRV8833Driver, MotorDriverError};
    use super::*;

    #[derive(Debug)]
    pub struct OutputPinMock {
        sleep: Option<PinState>,
    }

    impl OutputPinMock {
        fn new() -> Self {
            Self {
                sleep: None
            }
        }
    }

    #[derive(Debug)]
    pub enum MockError {}

    impl embedded_hal::digital::Error for MockError {
        fn kind(&self) -> ErrorKind {
            todo!()
        }
    }

    impl ErrorType for OutputPinMock { type Error = MockError; }

    impl OutputPin for OutputPinMock {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.sleep = Some(PinState::Low);

            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.sleep = Some(PinState::High);

            Ok(())
        }
    }

    #[test]
    fn it_toggles_sleep_for_sync_driver() -> Result<(), MotorDriverError<MockError>> {
        let in1 = OutputPinMock::new();
        let in2 = OutputPinMock::new();
        let in3 = OutputPinMock::new();
        let in4 = OutputPinMock::new();
        let sleep = OutputPinMock::new();

        let mut motor = DRV8833Driver::new_sync(in1, in2, in3, in4, sleep);

        // motor.sleep()?;
        // assert_eq!(motor.sleep.sleep, Some(PinState::Low));
        //
        // motor.wakeup()?;
        // assert_eq!(motor.sleep.sleep, Some(PinState::High));

        Ok(())
    }

    #[test]
    fn it_toggles_sleep_for_parallel_driver() -> Result<(), MotorDriverError<MockError>> {
        let in1 = OutputPinMock::new();
        let in2 = OutputPinMock::new();
        let in3 = OutputPinMock::new();
        let in4 = OutputPinMock::new();
        let sleep = OutputPinMock::new();

        let mut motor = DRV8833Driver::new_parallel(in1, in2, in3, in4, sleep);
        //
        // motor.sleep()?;
        // assert_eq!(motor.sleep.sleep, Some(PinState::Low));
        //
        // motor.wakeup()?;
        // assert_eq!(motor.sleep.sleep, Some(PinState::High));

        Ok(())
    }

    #[test]
    fn it_test_() -> Result<(), MotorDriverError<MockError>> {
        let in1 = OutputPinMock::new();
        let in2 = OutputPinMock::new();
        let in3 = OutputPinMock::new();
        let in4 = OutputPinMock::new();
        let sleep = OutputPinMock::new();

        let mut motor = DRV8833Driver::new_parallel(in1, in2, in3, in4, sleep);

        Ok(())
    }
}
