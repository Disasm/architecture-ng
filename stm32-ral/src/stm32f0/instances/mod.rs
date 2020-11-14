#[cfg(any(feature="doc", feature="stm32f0x0", feature="stm32f0x1", feature="stm32f0x2", feature="stm32f0x8"))]
pub mod crc;

#[cfg(any(feature="doc", feature="stm32f0x1", feature="stm32f0x2", feature="stm32f0x8"))]
pub mod gpio;
