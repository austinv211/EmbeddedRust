#![deny(unsafe_code)]
#![no_std]
#![no_main]

use aux5::entry;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    loop {}
}
