#![no_std]

#[cfg(feature = "stm32f0x2")]
pub mod stm32f0x2;

#[cfg(feature = "stm32f0x2")]
pub use stm32f0x2::*;
