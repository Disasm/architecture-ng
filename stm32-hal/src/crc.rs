//! CRC

use crate::pac::CRC;
use stm32_ral::crc::{self, Instance};
use stm32_ral::{read_reg, write_reg};
use crate::rcc::{Enable, AHB};

/// Extension trait to constrain the CRC peripheral
pub trait CrcExt {
    /// Constrains the CRC peripheral to play nicely with the other abstractions
    fn new(self, ahb: &mut AHB) -> Crc;
}

impl CrcExt for CRC {
    fn new(self, ahb: &mut AHB) -> Crc {
        CRC::enable(ahb);
        Crc { crc: self.into() }
    }
}

/// Constrained CRC peripheral
pub struct Crc {
    crc: Instance,
}

impl Crc {
    pub fn read(&self) -> u32 {
        read_reg!(crc, self.crc, DR)
    }

    pub fn write(&mut self, val: u32) {
        write_reg!(crc, self.crc, DR, val);
    }

    pub fn reset(&self) {
        write_reg!(crc, self.crc, CR, RESET: 1);
        // calling CRC::dr::write() just after CRC::cr::reset() will not work as expected, and
        // inserting single nop() seems to solve the problem.
        cortex_m::asm::nop();
    }

    pub fn free(self) -> CRC {
        crc::CRC::release(self.crc)
    }
}
