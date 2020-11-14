//! stm32ral module for stm32f0x2

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub use stm32ral::stm32f0::stm32f0x2::interrupts;
pub use stm32ral::stm32f0::stm32f0x2::Interrupt;
pub use stm32ral::stm32f0::stm32f0x2::interrupt;

pub use super::instances::crc;
pub use super::instances::gpio;

#[allow(non_snake_case)]
pub struct Peripherals {
    pub CRC: crc::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
}

impl Peripherals {
    pub unsafe fn steal() -> Self {
        let p = stm32_peripherals::stm32f0x2::Peripherals::steal();
        Peripherals {
            CRC: p.CRC.into(),
            GPIOA: p.GPIOA.into(),
            GPIOB: p.GPIOB.into(),
        }
    }
}
