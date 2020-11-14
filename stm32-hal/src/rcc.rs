// /// Extension trait that constrains the `RCC` peripheral
// pub trait RccExt {
//     /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
//     fn constrain(self) -> Rcc;
// }
//
// impl RccExt for RCC {
//     fn constrain(self) -> Rcc {
//         Rcc {
//             ahb: AHB { _0: () },
//             apb1: APB1 { _0: () },
//             apb2: APB2 { _0: () },
//             cfgr: CFGR {
//                 hse: None,
//                 hclk: None,
//                 pclk1: None,
//                 pclk2: None,
//                 sysclk: None,
//                 adcclk: None,
//             },
//             bkp: BKP { _0: () },
//         }
//     }
// }

/// Constrained RCC peripheral
///
/// Aquired by calling the [constrain](../trait.RccExt.html#tymethod.constrain) method
/// on the Rcc struct from the `PAC`
///
/// ```rust
/// let dp = pac::Peripherals::take().unwrap();
/// let mut rcc = dp.RCC.constrain();
/// ```
pub struct Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub ahb: AHB,
}

impl Rcc {
    pub fn new() -> Rcc {
        Rcc {
            ahb: AHB { _0: () }
        }
    }
}

pub struct AHB {
    _0: (),
}

pub(crate) mod sealed {
    /// Bus associated to peripheral
    pub trait RccBus {
        /// Bus type;
        type Bus;
    }
}
use sealed::RccBus;

/// Enable/disable peripheral
pub trait Enable: RccBus {
    fn enable(apb: &mut Self::Bus);
    fn disable(apb: &mut Self::Bus);
}

/// Reset peripheral
pub trait Reset: RccBus {
    fn reset(apb: &mut Self::Bus);
}

impl RccBus for crate::pac::CRC {
    type Bus = AHB;
}

impl Enable for crate::pac::CRC {
    fn enable(apb: &mut Self::Bus) {
        unimplemented!()
    }

    fn disable(apb: &mut Self::Bus) {
        unimplemented!()
    }
}
