#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose I/Os
//!
//! Used by: stm32f0x1, stm32f0x2, stm32f0x8

use core::marker::PhantomData;

pub use crate::stm32f0::peripherals::gpio::{RegisterBlock, ResetValues};
pub use crate::stm32f0::peripherals::gpio::{
    AFRH, AFRL, BRR, BSRR, IDR, LCKR, MODER, ODR, OSPEEDR, OTYPER, PUPDR,
};

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

/// Access functions for the GPIOA peripheral instance
pub mod GPIOA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    use stm32_peripherals::stm32f0x2 as p;
    use stm32_peripherals::traits::PeripheralAddress;

    // #[cfg(not(feature = "nosync"))]
    // const INSTANCE: Instance = Instance {
    //     addr: 0x48000000,
    //     _marker: ::core::marker::PhantomData,
    // };

    /// Reset values for each field in GPIOA
    pub const reset: ResetValues = ResetValues {
        MODER: 0x28000000,
        OTYPER: 0x00000000,
        OSPEEDR: 0x00000000,
        PUPDR: 0x24000000,
        IDR: 0x00000000,
        ODR: 0x00000000,
        BSRR: 0x00000000,
        LCKR: 0x00000000,
        AFRL: 0x00000000,
        AFRH: 0x00000000,
        BRR: 0x00000000,
    };

    impl From<p::GPIOA> for Instance {
        #[inline(always)]
        fn from(_v: p::GPIOA) -> Instance {
            Instance {
                addr: p::GPIOA::ADDRESS as u32,
                _marker: ::core::marker::PhantomData,
            }
        }
    }

    // #[cfg(not(feature = "nosync"))]
    // #[allow(renamed_and_removed_lints)]
    // #[allow(private_no_mangle_statics)]
    // #[no_mangle]
    // static mut GPIOA_TAKEN: bool = false;
    //
    // /// Safe access to GPIOA
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
    //         if GPIOA_TAKEN {
    //             None
    //         } else {
    //             GPIOA_TAKEN = true;
    //             Some(INSTANCE)
    //         }
    //     })
    // }
    //
    /// Release exclusive access to GPIOA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) -> p::GPIOA {
        if inst.addr == p::GPIOA::ADDRESS as u32 {
            unsafe { p::GPIOA::conjure() }
        } else {
            panic!("Released a peripheral which was not taken");
        }
    }
    //
    // /// Unsafely steal GPIOA
    // ///
    // /// This function is similar to take() but forcibly takes the
    // /// Instance, marking it as taken irregardless of its previous
    // /// state.
    // #[cfg(not(feature = "nosync"))]
    // #[inline]
    // pub unsafe fn steal() -> Instance {
    //     GPIOA_TAKEN = true;
    //     INSTANCE
    // }
}

/// Raw pointer to GPIOA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOA: *const RegisterBlock = 0x48000000 as *const _;

/// Access functions for the GPIOB peripheral instance
pub mod GPIOB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    use stm32_peripherals::stm32f0x2 as p;
    use stm32_peripherals::traits::PeripheralAddress;

    // #[cfg(not(feature = "nosync"))]
    // const INSTANCE: Instance = Instance {
    //     addr: 0x48000400,
    //     _marker: ::core::marker::PhantomData,
    // };

    /// Reset values for each field in GPIOB
    pub const reset: ResetValues = ResetValues {
        MODER: 0x00000000,
        OTYPER: 0x00000000,
        OSPEEDR: 0x00000000,
        PUPDR: 0x00000000,
        IDR: 0x00000000,
        ODR: 0x00000000,
        BSRR: 0x00000000,
        LCKR: 0x00000000,
        AFRL: 0x00000000,
        AFRH: 0x00000000,
        BRR: 0x00000000,
    };

    impl From<p::GPIOB> for Instance {
        #[inline(always)]
        fn from(_v: p::GPIOB) -> Instance {
            Instance {
                addr: p::GPIOB::ADDRESS as u32,
                _marker: ::core::marker::PhantomData,
            }
        }
    }

    // #[cfg(not(feature = "nosync"))]
    // #[allow(renamed_and_removed_lints)]
    // #[allow(private_no_mangle_statics)]
    // #[no_mangle]
    // static mut GPIOB_TAKEN: bool = false;
    //
    // /// Safe access to GPIOB
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
    //         if GPIOB_TAKEN {
    //             None
    //         } else {
    //             GPIOB_TAKEN = true;
    //             Some(INSTANCE)
    //         }
    //     })
    // }
    //
    /// Release exclusive access to GPIOB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) -> p::GPIOB {
        if inst.addr == p::GPIOB::ADDRESS as u32 {
            unsafe { p::GPIOB::conjure() }
        } else {
            panic!("Released a peripheral which was not taken");
        }
    }
    //
    // /// Unsafely steal GPIOB
    // ///
    // /// This function is similar to take() but forcibly takes the
    // /// Instance, marking it as taken irregardless of its previous
    // /// state.
    // #[cfg(not(feature = "nosync"))]
    // #[inline]
    // pub unsafe fn steal() -> Instance {
    //     GPIOB_TAKEN = true;
    //     INSTANCE
    // }
}

/// Raw pointer to GPIOB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOB: *const RegisterBlock = 0x48000400 as *const _;
