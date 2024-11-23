#![no_std]
#![no_main]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

// now use write volatile
#[entry]
fn main() -> ! {
    aux7::init();
    
    unsafe {
        // A magic address
        const GPIOE_BSRR: u32 = 0x48001018;
        
        // turn on the "North" red LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        
        // turn on the "East" green LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        
        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        
        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
    }
    
    loop {}
}