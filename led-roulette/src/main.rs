#![deny(unsafe_code)]
#![no_std]
#![no_main]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 50_u16;
    let v_half_period = Volatile::new(&mut half_period);
    const NUM_AHEAD: usize = 4;

    loop {
        for i in 0..8 {
            for next_add in 1..(NUM_AHEAD + 1) {
                let next = (i + next_add) % 8;

                leds[next].on().ok();
            }
            delay.delay_ms(v_half_period.read());
            leds[i].off().ok();
            delay.delay_ms(v_half_period.read());
        }
    }
}
