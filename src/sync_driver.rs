use embedded_hal::digital::OutputPin;
use crate::bridge::Bridge;

use crate::driver::Driver;

pub struct SyncDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pub a: Bridge<IN1, IN2>,
    pub b: Bridge<IN3, IN4>,
}

impl<IN1: OutputPin, IN2: OutputPin, IN3: OutputPin, IN4: OutputPin> SyncDriver<IN1, IN2, IN3, IN4> {
    pub fn new(in1: IN1, in2: IN2, in3: IN3, in4: IN4) -> Self {
        Self {
            a: Bridge::new(in1, in2),
            b: Bridge::new(in3, in4),
        }
    }
}

impl<IN1, IN2, IN3, IN4> Driver for SyncDriver<IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin {}
