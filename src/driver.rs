use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use embedded_hal::digital::OutputPin;

use crate::parallel_driver::ParallelDriver;
use crate::sync_driver::SyncDriver;

pub trait Driver {}

pub struct DRV8833Driver<DRIVER, SLEEP, EspError>
    where
        DRIVER: Driver,
        SLEEP: OutputPin<Error=EspError>,
{
    driver: DRIVER,
    sleep: SLEEP,
}

impl<DRIVER, SLEEP, EspError> Deref for DRV8833Driver<DRIVER, SLEEP, EspError>
    where
        DRIVER: Driver,
        SLEEP: OutputPin<Error=EspError>,
{
    type Target = DRIVER;

    fn deref(&self) -> &Self::Target {
        &self.driver
    }
}

impl<DRIVER, SLEEP, EspError> DerefMut for DRV8833Driver<DRIVER, SLEEP, EspError>
    where
        DRIVER: Driver,
        SLEEP: OutputPin<Error=EspError>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.driver
    }
}

impl<IN1, IN2, IN3, IN4, SLEEP, EspError> DRV8833Driver<SyncDriver<IN1, IN2, IN3, IN4>, SLEEP, EspError>
    where
        IN1: OutputPin<Error=EspError>,
        IN2: OutputPin<Error=EspError>,
        IN3: OutputPin<Error=EspError>,
        IN4: OutputPin<Error=EspError>,
        SLEEP: OutputPin<Error=EspError>,
{
    pub fn new_sync(in1: IN1, in2: IN2, in3: IN3, in4: IN4, sleep: SLEEP) -> DRV8833Driver<SyncDriver<IN1, IN2, IN3, IN4>, SLEEP, EspError>
        where
            IN1: OutputPin,
            IN2: OutputPin,
            IN3: OutputPin,
            IN4: OutputPin,
    {
        DRV8833Driver {
            driver: SyncDriver::new(in1, in2, in3, in4),
            sleep,
        }
    }
}

impl<IN1, IN2, IN3, IN4, SLEEP, EspError> DRV8833Driver<ParallelDriver<IN1, IN2, IN3, IN4>, SLEEP, EspError>
    where
        IN1: OutputPin<Error=EspError>,
        IN2: OutputPin<Error=EspError>,
        IN3: OutputPin<Error=EspError>,
        IN4: OutputPin<Error=EspError>,
        SLEEP: OutputPin<Error=EspError>,
{
    /// Creates a new motor driver instance configured for parallel mode, where both bridges are
    /// controlled identically. This mode is useful for two main purposes:
    /// 1. Increasing current output: By connecting IN1 with IN3 and IN2 with IN4, you can effectively
    ///    double the current output capability, as both bridges operate together to drive the motor.
    /// 2. Ensuring identical behavior: If you need both bridges to behave exactly the same, parallel
    ///    mode ensures synchronous control of the motor.
    pub fn new_parallel(
        in1: IN1,
        in2: IN2,
        in3: IN3,
        in4: IN4,
        sleep: SLEEP,
    ) -> DRV8833Driver<ParallelDriver<IN1, IN2, IN3, IN4>, SLEEP, EspError>
        where
            IN1: OutputPin<Error=EspError>,
            IN2: OutputPin<Error=EspError>,
            IN3: OutputPin<Error=EspError>,
            IN4: OutputPin<Error=EspError>,
    {
        DRV8833Driver {
            driver: ParallelDriver::new(in1, in2, in3, in4),
            sleep,
        }
    }
}

impl<DRIVER, SLEEP, EspError> DRV8833Driver<DRIVER, SLEEP, EspError>
    where
        DRIVER: Driver,
        SLEEP: OutputPin<Error=EspError>,
        EspError: Debug
{
    pub fn sleep(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.sleep.set_low().map_err(|error| MotorDriverError::EspError(error))
    }

    pub fn wakeup(&mut self) -> Result<(), MotorDriverError<EspError>> {
        self.sleep.set_high().map_err(|error| MotorDriverError::EspError(error))
    }

    pub fn is_fault(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum MotorDriverError<EspError: Debug> {
    EspError(EspError),
}

pub trait MotorDriver<EspError: Debug> {
    fn forward(&mut self) -> Result<(), MotorDriverError<EspError>>;
    fn reverse(&mut self) -> Result<(), MotorDriverError<EspError>>;

    /// Sets the motor driver to coast mode, allowing the motor to freely spin or coast to a stop
    /// without applying any active driving or braking force. In this mode, both the forward and
    /// reverse inputs are set low, disconnecting the motor from the driver circuitry. This allows
    /// the motor to naturally decelerate and come to a stop based on its inertia or external forces.
    /// Coast mode is useful when a smooth and natural deceleration of the motor is desired, such as
    /// when transitioning between motor states or when manual control requires the motor to spin
    /// freely without any active driving or braking.
    fn coast(&mut self) -> Result<(), MotorDriverError<EspError>>;

    /// Sets the motor driver to stop mode, causing the motor to rapidly come to a halt by
    /// applying a fast decay to the current in the motor winding. In fast decay, the magnetic field
    /// around the motor winding collapses quickly when the motor driver switches off, resulting in
    /// rapid deceleration. This mode is beneficial for achieving fast motor response times and
    /// transitioning between motor states quickly. However, it may produce higher levels of
    /// electrical noise due to the rapid changes in current. Use stop mode when immediate stopping
    /// of the motor is required, accepting the trade-off of potential electrical noise.
    fn stop(&mut self) -> Result<(), MotorDriverError<EspError>>;
}
