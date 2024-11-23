#![no_std]
#![no_main]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();
    
    unsafe {
        // A magic address
        const GPIOE_BSRR: u32 = 0x48001018;
        
        // turn on the "North" red LED
        *(GPIOE_BSRR as *mut u32) = 1 << 9;
        
        // turn on the "East" green LED
        *(GPIOE_BSRR as *mut u32) = 1 << 11;
        
        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
        
        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);  
    }
    
    loop {}
}