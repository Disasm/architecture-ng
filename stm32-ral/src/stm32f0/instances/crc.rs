#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! cyclic redundancy check calculation unit
//!
//! Used by: stm32f0x0, stm32f0x1, stm32f0x2, stm32f0x8

use core::marker::PhantomData;

pub use stm32ral::stm32f0::peripherals::crc::{RegisterBlock, ResetValues};
pub use stm32ral::stm32f0::peripherals::crc::{CR, DR, IDR, INIT};

#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

/// Access functions for the CRC peripheral instance
pub mod CRC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    use stm32_peripherals::stm32f0x2 as p;
    use stm32_peripherals::traits::PeripheralAddress;

    // #[cfg(not(feature = "nosync"))]
    // const INSTANCE: Instance = Instance {
    //     addr: 0x40023000,
    //     _marker: ::core::marker::PhantomData,
    // };

    /// Reset values for each field in CRC
    pub const reset: ResetValues = ResetValues {
        DR: 0xFFFFFFFF,
        IDR: 0x00000000,
        CR: 0x00000000,
        INIT: 0xFFFFFFFF,
    };

    impl From<p::CRC> for Instance {
        #[inline(always)]
        fn from(_v: p::CRC) -> Instance {
            Instance {
                addr: p::CRC::ADDRESS as u32,
                _marker: ::core::marker::PhantomData,
            }
        }
    }

    // #[cfg(not(feature = "nosync"))]
    // #[allow(renamed_and_removed_lints)]
    // #[allow(private_no_mangle_statics)]
    // #[no_mangle]
    // static mut CRC_TAKEN: bool = false;

    // /// Safe access to CRC
    // ///
    // /// This function returns `Some(Instance)` if this instance is not
    // /// currently taken, and `None` if it is. This ensures that if you
    // /// do get `Some(Instance)`, you are ensured unique access to
    // /// the peripheral and there cannot be data races (unless other
    // /// code uses `unsafe`, of course). You can then pass the
    // /// `Instance` around to other functions as required. When you're
    // /// done with it, you can call `release(instance)` to return it.
    // ///
    // /// `Instance` itself dereferences to a `RegisterBlock`, which
    // /// provides access to the peripheral's registers.
    // #[cfg(not(feature = "nosync"))]
    // #[inline]
    // pub fn take() -> Option<Instance> {
    //     external_cortex_m::interrupt::free(|_| unsafe {
    //         if CRC_TAKEN {
    //             None
    //         } else {
    //             CRC_TAKEN = true;
    //             Some(INSTANCE)
    //         }
    //     })
    // }
    //
    /// Release exclusive access to CRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) -> p::CRC {
        if inst.addr == p::CRC::ADDRESS as u32 {
            unsafe { p::CRC::conjure() }
        } else {
            panic!("Released a peripheral which was not taken");
        }
    }
    //
    // /// Unsafely steal CRC
    // ///
    // /// This function is similar to take() but forcibly takes the
    // /// Instance, marking it as taken irregardless of its previous
    // /// state.
    // #[cfg(not(feature = "nosync"))]
    // #[inline]
    // pub unsafe fn steal() -> Instance {
    //     CRC_TAKEN = true;
    //     INSTANCE
    // }
}

/// Raw pointer to CRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRC: *const RegisterBlock = 0x40023000 as *const _;
