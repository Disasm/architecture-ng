use core::marker::PhantomData;
use peripheral_traits::PeripheralAddress;

#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}

impl CRC {
    #[inline(always)]
    /// Unsafely obtains an instance of CRC
    ///
    /// It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    pub unsafe fn conjure() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}

impl PeripheralAddress for CRC {
    const ADDRESS: *const () = 0x4002_3000 as _;
}

#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}

impl GPIOA {
    #[inline(always)]
    /// Unsafely obtains an instance of GPIOA
    ///
    /// It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    pub unsafe fn conjure() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}

impl PeripheralAddress for GPIOA {
    const ADDRESS: *const () = 0x4800_0000 as _;
}

#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}

impl GPIOB {
    #[inline(always)]
    /// Unsafely obtains an instance of GPIOB
    ///
    /// It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    pub unsafe fn conjure() -> Self {
        Self {
            _marker: PhantomData
        }
    }
}

impl PeripheralAddress for GPIOB {
    const ADDRESS: *const () = 0x4800_0400 as _;
}


#[cfg(feature="safe-api")]
#[no_mangle]
pub(crate) static mut DEVICE_PERIPHERALS: bool = false;

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

impl Peripherals {
    #[cfg(feature="safe-api")]
    #[inline]
    /// Returns all the peripherals *once*
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }

    #[cfg(feature="safe-api")]
    #[inline(always)]
    /// Unchecked version of Peripherals::take
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals::conjure()
    }

    #[inline(always)]
    /// Unsafely constructs `Peripherals`
    ///
    /// It is the caller's responsibility to use the returned instance
    /// in a safe way that does not conflict with other instances.
    pub unsafe fn conjure() -> Peripherals {
        Peripherals {
            CRC: CRC::conjure(),
            GPIOA: GPIOA::conjure(),
            GPIOB: GPIOB::conjure(),
        }
    }
}
