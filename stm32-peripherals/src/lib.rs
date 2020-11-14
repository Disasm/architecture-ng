#![no_std]

pub use peripheral_traits as traits;

pub mod generic;

pub mod stm32f0x2;
//pub mod stm32f103;

#[cfg(feature = "stm32f0x2")]
pub use stm32f0x2::*;
