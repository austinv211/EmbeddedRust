#![no_std]

pub use cortex_m_rt::entry;
pub use panic_itm;

pub use stm32f3_discovery::stm32f3xx_hal::prelude::*;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

pub fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}
