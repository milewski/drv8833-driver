//! This crate provides a driver for the [DRV8833 Dual Bridge Motor Driver](https://www.ti.com/lit/ds/symlink/drv8833.pdf).
//!
//! The motor driver supports two motors and includes a standby pin and a fault interrupt that goes low when overheating or overcurrent conditions are detected.
//!
//! ### This driver offers several operating modes:
//!
//! #### [`Split`](MotorDriver::new_split)
//! Enables independent control over each bridge (A and B).
//!
//! #### [`Parallel`](MotorDriver::new_parallel)
//! Treats both bridges as a single unit, effectively doubling the current when connected in parallel.
//!
//! #### [`PWM Split`](MotorDriver::new_pwm_split)
//! Allows individual control over each bridge using PWM signals.
//!
//! #### [`PWM Split Single`](MotorDriver::new_pwm_split_single)
//! Allows individual control over each bridge and uses a single PWM signal shared applied directly to eep pin.
//!
//! #### [`PWM Parallel`](MotorDriver::new_pwm_parallel)
//! Controls both bridges simultaneously with a single PWM signal.
mod bridge;
mod driver;
mod parallel_driver;
mod split_driver;
mod pwm_parallel_driver;
mod pwm_split_driver;

pub use driver::*;
