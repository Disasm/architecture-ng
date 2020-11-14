use stm32_peripherals::stm32f0x2 as p;
use stm32_peripherals::traits::PeripheralAddress;
pub use stm32f0::stm32f0x2::crc;
pub use stm32f0::stm32f0x2::gpioa;
use core::ops::Deref;

#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC(p::CRC);

impl CRC {
    #[inline(always)]
    pub fn into_inner(self) -> p::CRC { self.0 }
}

impl From<p::CRC> for CRC {
    #[inline(always)]
    fn from(v: p::CRC) -> Self { CRC(v) }
}

impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(p::CRC::ADDRESS as *const crc::RegisterBlock) }
    }
}


#[doc = "General-purpose I/Os"]
pub struct GPIOA(p::GPIOA);

impl From<p::GPIOA> for GPIOA {
    #[inline(always)]
    fn from(v: p::GPIOA) -> Self { GPIOA(v) }
}

impl From<GPIOA> for p::GPIOA {
    #[inline(always)]
    fn from(v: GPIOA) -> Self { v.0 }
}

impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(p::GPIOA::ADDRESS as *const gpioa::RegisterBlock) }
    }
}


#[doc = "General-purpose I/Os"]
pub struct GPIOB(p::GPIOB);

impl From<p::GPIOB> for GPIOB {
    #[inline(always)]
    fn from(v: p::GPIOB) -> Self { GPIOB(v) }
}

impl From<GPIOB> for p::GPIOB {
    #[inline(always)]
    fn from(v: GPIOB) -> Self { v.0 }
}

impl Deref for GPIOB {
    type Target = stm32f0::stm32f0x2::gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(p::GPIOB::ADDRESS as *const stm32f0::stm32f0x2::gpioa::RegisterBlock) }
    }
}


#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
}

impl From<p::Peripherals> for Peripherals {
    #[inline(always)]
    fn from(p: p::Peripherals) -> Self {
        Peripherals {
            CRC: CRC(p.CRC),
            GPIOA: GPIOA(p.GPIOA),
            GPIOB: GPIOB(p.GPIOB),
        }
    }
}
