# Rust Driver for DRV8833 Dual Bridge Motor Driver

[![Build](https://github.com/milewski/drv8833-driver/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/milewski/drv8833-driver/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/drv8833-driver.svg)](https://crates.io/crates/drv8833-driver)
[![Documentation](https://docs.rs/drv8833-driver/badge.svg)](https://docs.rs/drv8833-driver)

Driver for the DRV8833 motor driver, supporting the operation of the motor in various modes.
See the [documentation](https://docs.rs/drv8833-driver) for more details.

For detailed information on the DRV8833, refer to the [datasheet](https://www.ti.com/lit/ds/symlink/drv8833.pdf).

# Installation

You can install the package via Cargo:

```sh
cargo add drv8833-driver
```

## Usage

Below is an example demonstrating how to use the driver with the [esp-idf-hal](https://crates.io/crates/esp-idf-hal) crate:

```rust
use esp_idf_hal::gpio::{AnyInputPin, Input, PinDriver};
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::prelude::Peripherals;

use drv8833_driver::{MotorDriver, PwmMovement};

fn main() -> anyhow::Result<()> {
    // Initialize peripherals
    let peripherals = Peripherals::take()?;

    // Initialize GPIO pins
    let in1 = PinDriver::output(peripherals.pins.gpio1)?;
    let in2 = PinDriver::output(peripherals.pins.gpio2)?;
    let in3 = PinDriver::output(peripherals.pins.gpio3)?;
    let in4 = PinDriver::output(peripherals.pins.gpio4)?;

    // Initialize LEDC timer and driver
    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default())?;
    let pwm = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio5)?;

    // Initialize motor driver
    let motor = MotorDriver::new_pwm_parallel(
        in1, in2, in3, in4, pwm, None::<PinDriver<AnyInputPin, Input>>,
    );

    // Control the motor
    motor.forward(100)?;
    motor.reverse(100)?;
    motor.stop()?;
    motor.coast()?;

    Ok(())
}
```

## License

The MIT License (MIT). Please see [License File](./LICENSE) for more information.