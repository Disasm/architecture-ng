#![no_std]

pub use stm32ral::{read_reg, write_reg, modify_reg, reset_reg};
pub use stm32ral::{RORegister, UnsafeRORegister};
pub use stm32ral::{WORegister, UnsafeWORegister};
pub use stm32ral::{RWRegister, UnsafeRWRegister};

#[cfg(any(feature="doc", feature="stm32f0x2"))]
pub mod stm32f0;

#[cfg(feature="stm32f0x2")]
pub use stm32f0::stm32f0x2::*;
