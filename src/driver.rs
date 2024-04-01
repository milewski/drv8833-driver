use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::pwm::SetDutyCycle;

use crate::parallel_driver::ParallelDriver;
use crate::pwm_parallel_driver::PwmParallelDriver;
use crate::pwm_split_driver::PwmSplitDriver;
use crate::split_driver::SplitDriver;

pub type PwmParallelDriverType<IN1, IN2, IN3, IN4, PWM, FAULT> = MotorDriver<PwmParallelDriver<IN1, IN2, IN3, IN4, Arc<Mutex<PWM>>>, Arc<Mutex<PWM>>, FAULT>;
pub type PwmSplitDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT> = MotorDriver<PwmSplitDriver<IN1, IN2, IN3, IN4>, Option<SLEEP>, FAULT>;
pub type PwmSplitSingleDriverType<IN1, IN2, IN3, IN4, PWM, FAULT> = MotorDriver<SplitDriver<IN1, IN2, IN3, IN4>, PWM, FAULT>;
pub type SplitDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT> = MotorDriver<SplitDriver<IN1, IN2, IN3, IN4>, Option<SLEEP>, FAULT>;
pub type ParallelDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT> = MotorDriver<ParallelDriver<IN1, IN2, IN3, IN4>, Option<SLEEP>, FAULT>;

/// Generics trait implemented by all drive modes.
pub trait Driver {}

/// Represents a motor driver, providing access to various modes of operation.
///
/// This struct facilitates the creation of four different modes:
///
/// - [`MotorDriver::new_split`]: Enables control over each bridge (A and B) independently.
/// - [`MotorDriver::new_parallel`]: Treats both bridges as a single unit, effectively doubling the current when connected in parallel.
/// - [`MotorDriver::new_pwm_split`]: Allows individual control over each bridge using PWM signals.
/// - [`MotorDriver::new_pwm_split_single`]: Allows individual control over each bridge while using a single PWM signal over the eep pin.
/// - [`MotorDriver::new_pwm_parallel`]: Controls both bridges simultaneously with a single PWM signal.
pub struct MotorDriver<DRIVER: Driver, SLEEP, FAULT: InputPin> {
    driver: DRIVER,
    sleep: SLEEP,
    fault: Option<FAULT>,
}

impl<DRIVER: Driver, SLEEP, FAULT: InputPin> Deref for MotorDriver<DRIVER, SLEEP, FAULT> {
    type Target = DRIVER;

    fn deref(&self) -> &Self::Target {
        &self.driver
    }
}

impl<DRIVER: Driver, SLEEP, FAULT: InputPin> DerefMut for MotorDriver<DRIVER, SLEEP, FAULT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.driver
    }
}

impl<IN1, IN2, IN3, IN4, SLEEP, FAULT> SplitDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        SLEEP: OutputPin,
        FAULT: InputPin,
{
    /// Creates a new [MotorDriver] instance with split control mode.
    ///
    /// In split mode, each bridge of the motor driver can be controlled independently.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use drv8833_driver::driver::MotorDriver;
    ///
    /// let in1 = PinDriver::output(peripherals.pins.gpio5)?;
    /// let in2 = PinDriver::output(peripherals.pins.gpio4)?;
    /// let in3 = PinDriver::output(peripherals.pins.gpio18)?;
    /// let in4 = PinDriver::output(peripherals.pins.gpio19)?;
    /// let sleep = PinDriver::output(peripherals.pins.gpio3)?;
    ///
    /// let mut motor = MotorDriver::new_split(
    ///     in1, in2, in3, in4, Some(sleep), None::<PinDriver<AnyInputPin, Input>>,
    /// );
    /// ```
    pub fn new_split(
        in1: IN1,
        in2: IN2,
        in3: IN3,
        in4: IN4,
        sleep: Option<SLEEP>,
        fault: Option<FAULT>,
    ) -> SplitDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT>
        where
            IN1: OutputPin,
            IN2: OutputPin,
            IN3: OutputPin,
            IN4: OutputPin,
    {
        MotorDriver {
            driver: SplitDriver::new(in1, in2, in3, in4),
            sleep,
            fault,
        }
    }
}

impl<IN1, IN2, IN3, IN4, SLEEP, FAULT> PwmSplitDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT>
    where
        IN1: SetDutyCycle,
        IN2: SetDutyCycle,
        IN3: SetDutyCycle,
        IN4: SetDutyCycle,
        SLEEP: OutputPin,
        FAULT: InputPin,
{
    /// Creates a new [MotorDriver] instance in PWM split control mode.
    ///
    /// In this mode, each bridge pin is controlled individually via PWM signals. Note that using this mode
    /// may require a significant number of PWM channels, especially if multiple modules are in use. If your
    /// device has limited PWM channels available, you may prefer using [`MotorDriver::new_pwm_parallel`],
    /// which consumes fewer PWM channels but sacrifices individual control over both bridges.
    ///
    /// # Example:
    ///
    /// ```ignore
    /// use drv8833_driver::driver::MotorDriver;
    ///
    /// let sleep = PinDriver::output(peripherals.pins.gpio3)?;
    /// let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default())?;
    ///
    /// let in1 = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio5)?;
    /// let in2 = LedcDriver::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio4)?;
    /// let in3 = LedcDriver::new(peripherals.ledc.channel2, &timer, peripherals.pins.gpio18)?;
    /// let in4 = LedcDriver::new(peripherals.ledc.channel3, &timer, peripherals.pins.gpio19)?;
    ///
    /// let mut motor = MotorDriver::new_pwm_split(
    ///     in1, in2, in3, in4, Some(sleep), None::<PinDriver<AnyInputPin, Input>>,
    /// );
    /// ```
    pub fn new_pwm_split(
        in1: IN1,
        in2: IN2,
        in3: IN3,
        in4: IN4,
        sleep: Option<SLEEP>,
        fault: Option<FAULT>,
    ) -> PwmSplitDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT> {
        MotorDriver {
            driver: PwmSplitDriver::new(in1, in2, in3, in4),
            sleep,
            fault,
        }
    }
}

impl<IN1, IN2, IN3, IN4, SLEEP, FAULT> ParallelDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        SLEEP: OutputPin,
        FAULT: InputPin,
{
    /// Creates a new instance of `MotorDriver` in parallel control mode.
    ///
    /// In this mode, both bridges are driven with the same inputs, causing them to behave identically.
    /// The primary advantage of this mode is that it effectively doubles the current output when the bridges
    /// are physically connected in parallel, with IN1 connected to IN3 and IN2 connected to IN4.
    ///
    /// # Example:
    ///
    /// ```ignore
    /// use drv8833_driver::driver::MotorDriver;
    ///
    /// let in1 = PinDriver::output(peripherals.pins.gpio5)?;
    /// let in2 = PinDriver::output(peripherals.pins.gpio4)?;
    /// let in3 = PinDriver::output(peripherals.pins.gpio18)?;
    /// let in4 = PinDriver::output(peripherals.pins.gpio19)?;
    /// let sleep = PinDriver::output(peripherals.pins.gpio3)?;
    ///
    /// let mut motor = MotorDriver::new_parallel(
    ///     in1, in2, in3, in4, Some(sleep), None::<PinDriver<AnyInputPin, Input>>,
    /// );
    /// ```
    pub fn new_parallel(
        in1: IN1,
        in2: IN2,
        in3: IN3,
        in4: IN4,
        sleep: Option<SLEEP>,
        fault: Option<FAULT>,
    ) -> ParallelDriverType<IN1, IN2, IN3, IN4, SLEEP, FAULT>
        where
            IN1: OutputPin,
            IN2: OutputPin,
            IN3: OutputPin,
            IN4: OutputPin,
    {
        MotorDriver {
            driver: ParallelDriver::new(in1, in2, in3, in4),
            sleep,
            fault,
        }
    }
}

impl<IN1, IN2, IN3, IN4, PWM, FAULT> PwmParallelDriverType<IN1, IN2, IN3, IN4, PWM, FAULT>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        FAULT: InputPin,
{
    /// Creates a new [MotorDriver] instance with PWM parallel control mode.
    ///
    /// In PWM parallel mode, a single PWM channel is used to control all four bridge inputs.
    /// This allows simultaneous control of all inputs. It's useful for scenarios where a single
    /// motor needs increased current, achieved by connecting IN1 with IN3 and IN2 with IN4.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use drv8833_driver::driver::Motor;
    ///
    /// let in1 = PinDriver::output(peripherals.pins.gpio5)?;
    /// let in2 = PinDriver::output(peripherals.pins.gpio4)?;
    /// let in3 = PinDriver::output(peripherals.pins.gpio18)?;
    /// let in4 = PinDriver::output(peripherals.pins.gpio19)?;
    ///
    /// let timer = LedcTimerDriver::new(peripherals.ledc.timer3, &TimerConfig::default())?;
    /// let sleep = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio3)?;
    ///
    /// let mut motor = Motor::new_pwm_parallel(
    ///     in1, in2, in3, in4, sleep, None::<PinDriver<AnyInputPin, Input>>,
    /// );
    /// ```
    pub fn new_pwm_parallel(
        in1: IN1,
        in2: IN2,
        in3: IN3,
        in4: IN4,
        pwm: PWM,
        fault: Option<FAULT>,
    ) -> PwmParallelDriverType<IN1, IN2, IN3, IN4, PWM, FAULT>
        where
            IN1: OutputPin,
            IN2: OutputPin,
            IN3: OutputPin,
            IN4: OutputPin,
    {
        let pwm = Arc::new(Mutex::new(pwm));

        MotorDriver {
            driver: PwmParallelDriver::new(in1, in2, in3, in4, pwm.clone()),
            sleep: pwm,
            fault,
        }
    }
}

impl<IN1, IN2, IN3, IN4, PWM, FAULT> PwmSplitSingleDriverType<IN1, IN2, IN3, IN4, PWM, FAULT>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        FAULT: InputPin,
{
    /// Creates a new [MotorDriver] instance with PWM split single control mode.
    ///
    /// In this mode, the PWM signal is applied to the eep PIN instead of each IN pin, thus
    /// conserving PWM channels.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use drv8833_driver::driver::Motor;
    ///
    /// let in1 = PinDriver::output(peripherals.pins.gpio5)?;
    /// let in2 = PinDriver::output(peripherals.pins.gpio4)?;
    /// let in3 = PinDriver::output(peripherals.pins.gpio18)?;
    /// let in4 = PinDriver::output(peripherals.pins.gpio19)?;
    ///
    /// let timer = LedcTimerDriver::new(peripherals.ledc.timer1, &TimerConfig::default())?;
    /// let pwm = LedcDriver::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio3)?;
    ///
    /// let mut motor = Motor::new_pwm_split_single(
    ///     in1, in2, in3, in4, pwm, None::<PinDriver<AnyInputPin, Input>>,
    /// );
    /// ```
    pub fn new_pwm_split_single(
        in1: IN1,
        in2: IN2,
        in3: IN3,
        in4: IN4,
        pwm: PWM,
        fault: Option<FAULT>,
    ) -> PwmSplitSingleDriverType<IN1, IN2, IN3, IN4, PWM, FAULT>
        where
            IN1: OutputPin,
            IN2: OutputPin,
            IN3: OutputPin,
            IN4: OutputPin,
    {
        MotorDriver {
            driver: SplitDriver::new(in1, in2, in3, in4),
            sleep: pwm,
            fault,
        }
    }
}

impl<DRIVER, SLEEP, FAULT> MotorDriver<DRIVER, SLEEP, FAULT>
    where
        DRIVER: Driver,
        SLEEP: SetDutyCycle,
        FAULT: InputPin,
{
    pub fn set_duty_cycle(&mut self, percent: u8) -> Result<(), MotorDriverError> {
        self.sleep.set_duty_cycle_percent(percent).map_err(|_| MotorDriverError::UnableToSetDuty)?;

        Ok(())
    }
}

impl<DRIVER, SLEEP, FAULT> MotorDriver<DRIVER, Option<SLEEP>, FAULT>
    where
        DRIVER: Driver,
        SLEEP: OutputPin,
        FAULT: InputPin,
{
    /// Puts the device into a low power sleep state, In this state, the H-bridges are disabled, the
    /// gate drive charge pump is stopped, all internal logic is reset, and all internal clocks are
    /// stopped. All inputs are ignored until [MotorDriver::wakeup] is called.
    pub fn sleep(&mut self) -> Result<(), MotorDriverError> {
        if let Some(sleep) = &mut self.sleep {
            sleep.set_low().map_err(|_| MotorDriverError::GpioError)
        } else {
            Ok(())
        }
    }

    /// Wake up the device from sleep mode.
    pub fn wakeup(&mut self) -> Result<(), MotorDriverError> {
        if let Some(sleep) = &mut self.sleep {
            sleep.set_high().map_err(|_| MotorDriverError::GpioError)
        } else {
            Ok(())
        }
    }
}

impl<DRIVER, PWM, FAULT> MotorDriver<DRIVER, PWM, FAULT>
    where
        DRIVER: Driver,
        FAULT: InputPin,
{
    /// Logic low when in fault condition (over-temperature, over-current)
    pub fn is_faulty(&mut self) -> Result<bool, MotorDriverError> {
        if let Some(fault) = &mut self.fault {
            fault.is_low().map_err(|_| MotorDriverError::GpioError)
        } else {
            Ok(false)
        }
    }
}

/// Represents all possible errors that may occur during the utilization of this crate.
#[derive(Debug)]
pub enum MotorDriverError {
    /// Returned when fail to set pin low/high.
    GpioError,
    /// Returned when fail to set duty value.
    UnableToSetDuty,
    /// Returned when we are unable to acquire mutex lock.
    PwmLocked,
    /// Returned when in PWM mode and a duty value is not within 0-100 range.
    InvalidRange,
}

/// A trait representing movement control for motors via PWM signal.
pub trait PwmMovement {
    /// Sets the motor direction to forward with a given percentage of speed.
    fn forward(&mut self, percent: u8) -> Result<(), MotorDriverError>;

    /// Sets the motor direction to reverse with a given percentage of speed.
    fn reverse(&mut self, percent: u8) -> Result<(), MotorDriverError>;
}

/// A trait representing movement control for motors.
pub trait Movement {
    /// This method instructs the motor to move in the forward direction.
    fn forward(&mut self) -> Result<(), MotorDriverError>;

    /// This method instructs the motor to move in the reverse direction.
    fn reverse(&mut self) -> Result<(), MotorDriverError>;
}

/// A trait representing braking control for motors.
pub trait Breaks {
    /// Sets the motor driver to coast mode, allowing the motor to freely spin or coast to a stop
    /// without applying any active driving or braking force. In this mode, both the forward and
    /// reverse inputs are set low, disconnecting the motor from the driver circuitry. This allows
    /// the motor to naturally decelerate and come to a stop based on its inertia or external forces.
    /// Coast mode is useful when a smooth and natural deceleration of the motor is desired, such as
    /// when transitioning between motor states or when manual control requires the motor to spin
    /// freely without any active driving or braking.
    fn coast(&mut self) -> Result<(), MotorDriverError>;

    /// Sets the motor driver to stop mode, causing the motor to rapidly come to a halt by
    /// applying a fast decay to the current in the motor winding. In fast decay, the magnetic field
    /// around the motor winding collapses quickly when the motor driver switches off, resulting in
    /// rapid deceleration. This mode is beneficial for achieving fast motor response times and
    /// transitioning between motor states quickly. However, it may produce higher levels of
    /// electrical noise due to the rapid changes in current. Use stop mode when immediate stopping
    /// of the motor is required, accepting the trade-off of potential electrical noise.
    fn stop(&mut self) -> Result<(), MotorDriverError>;
}
