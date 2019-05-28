//! STM32L4 HAL implementation
//!
//! NOTE: This HAL implementation is under active development (as is the underlying
//! `embedded_hal` itself, together with its traits, some of which are unproven).
//! We follow the HAL implementation in <https://github.com/therealprof/stm32f4xx-hal>
//! and pull in individual devices behind features - the goal is for this implementation
//! to become ubiquitous for the STM32L4 family of devices (well-tested, feature-complete,
//! well-documented). However! At this time, actual testing has only been performed for
//! the STM32L432KC microcontroller. Participation is of course very welcome!

#![no_std]

#[cfg(not(feature = "device-selected"))]
compile_error!("This crate requires one of the following features enabled:
       stm32l4x1
       stm32l4x2
       stm32l4x3
       stm32l4x4
       stm32l4x5
       stm32l4x6");

pub mod hal;

// can we remove this? if not, super/self from hal.rs does not
// seem to work :/
#[cfg(feature = "extra-traits")]
pub(crate) mod extra_traits;

pub use cortex_m;

pub use stm32l4;
#[cfg(feature = "stm32l4x1")]
pub use stm32l4::stm32l4x1 as pac;

#[cfg(feature = "stm32l4x2")]
pub use stm32l4::stm32l4x2 as pac;

#[cfg(feature = "stm32l4x3")]
pub use stm32l4::stm32l4x3 as pac;

#[cfg(feature = "stm32l4x5")]
pub use stm32l4::stm32l4x5 as pac;

#[cfg(feature = "stm32l4x6")]
pub use stm32l4::stm32l4x6 as pac;

#[cfg(feature = "rt")]
pub use self::pac::interrupt;

// TODO: get rid of these re-exports
// pub use crate::cortex_m as core;
pub use crate::pac as device;
pub use crate::pac as stm32;

#[cfg(feature = "device-selected")]
pub mod prelude;


pub mod datetime;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod delay;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod dma;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod flash;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod gpio;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod i2c;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod pwr;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod rcc;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod rng;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod rtc;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod serial;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod spi;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod time;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod timer;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod tsc;
#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
pub mod pwm;
