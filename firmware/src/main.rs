use stm32_peripherals::Peripherals;
use stm32_pal::GPIOA;
use stm32_hal::rcc::Rcc;
use stm32_hal::crc::*;

fn main() {
    let p = Peripherals::take().unwrap();

    // Use GPIO with stm32-pal
    let gpioa: GPIOA = p.GPIOA.into();
    gpioa.bsrr.write(|w| w.br0().set_bit());

    // Use HAL with stm32-ral
    let mut rcc = Rcc::new();
    let crc = p.CRC.new(&mut rcc.ahb);
    crc.reset();
}
