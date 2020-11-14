#![no_std]

/// Memory-access peripheral that has a fixed memory address
pub trait PeripheralAddress {
    const ADDRESS: *const ();
}

/// Memory-access peripheral with 16-bit access
pub trait Peripheral16 {}

/// Memory-access peripheral with 32-bit access
pub trait Peripheral32 {}
