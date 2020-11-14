//! Parent module for all STM32F0 devices.

/// Peripherals shared by multiple devices
pub use stm32ral::stm32f0::peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="stm32f0x2", feature="doc"))]
pub mod stm32f0x2;
