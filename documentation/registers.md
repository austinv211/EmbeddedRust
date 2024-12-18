# Registers

Exploring what the Led API does under the hood

In summary, it just writes to special memory regions

In the code below we added to main.rs...
```Rust
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}
```
The address `0x48001018` points to a register. A register is a special regions of memory that controlls a peripheral. A peripheral is a piece of electronics that sits right next to the processor within the microncontroller package and provides the processor with extra functionality. After all, the processor, on its owne can only do math and logic.

This particular register controls General Purpose Input/Output GPIO pins (GPIO is a peripheral) and can be used to drive each of those pins low or high.

Note: A microcontorller's pins are connected to the LEDs with the right polarity. All that we have to do is output some non-zero voltage through the pin to turn the LED on. The pins attached to the LEDs are configured as digital outputs and can only output two different voltage levels: low (0) or high (3) volts. These high and low states map directly to the concept of digital logic. This is why the pin configuration is known as digital output.

## Read the Reference Manual
The microncontroller has several pins. For convenience, these pins are grouped in ports of 16 pins. Each port is named with a letter. Port A, Port B, etc. and the pins within each port are named with numbers 0 to 15.

The first thing we have to do is find out which pin is connected to which LED. See the User manual for the F3 [here](http://www.st.com/resource/en/user_manual/dm00063382.pdf) Section 6.4 LEDS - Page 19

```
• LD1 PWR: Red LED indicates that the board is powered.
• LD2 COM: LD2 default status is red. LD2 turns to green to indicate that
communications are in progress between the PC and the ST-LINK/V2.
• User LD3: Red LED is a user LED connected to the I/O PE9 of the STM32F303VCT6.
• User LD4: Blue LED is a user LED connected to the I/O PE8 of the STM32F303VCT6.
• User LD5: Orange LED is a user LED connected to the I/O PE10 of the
STM32F303VCT6.
• User LD6: Green LED is a user LED connected to the I/O PE15 of the
STM32F303VCT6.
• User LD7: Green LED is a user LED connected to the I/O PE11 of the
STM32F303VCT6.
• User LD8: Orange LED is a user LED connected to the I/O PE14 of the
STM32F303VCT6.
• User LD9: Blue LED is a user LED connected to the I/O PE12 of the STM32F303VCT6.
• User LD10: Red LED is a user LED connected to the I/O PE13 of the
STM32F303VCT6
```

The manual sayas LD3 (The North LED) is connected to the pin PE9. PE9 is short form of Pin 9 on Port E.
LD7, The East LEAD, is connected to the pin PE11.

Up to this point, we know thaty we want to change the state of the pins PE9 and PE11 to turn the North/East LEDs on/off. These pins are part of Port E so we'll have to deal with the GPIOE peripheral.
Each peripheral has a register block associated to it. A register block is a collectioon of registers allocated in contiguous memory. The address at which the register block starts is known as its base address. To find the base address of the GPIOE peripheral can be found in the memory map section of the reference manual found [here](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)

The memory map table says that the base address of the GPIOE register block is 0x4800_1000

Each peripheral also has its own section in the documentation. Each of hese sections ends with a table of the registers that the peripheral's register block contains. For the GPIO family of peripheral, that table is in the GPIO register map section.

BSSR is the register which we will be using to set/reset. Its offset value is `0x18` from the base address of the GPIOE. We can look up BSSR in the reference manual under GPIO Registers -> GPIO port bit set/reset register (GPIOxBSSR)

The reference manual mentions this write only, so let's try reading it to see what happens.

```
(gdb) next
16              *(GPIOE_BSRR as *mut u32) = 1 << 9;

(gdb) x 0x48001018
0x48001018:     0x00000000

(gdb) # the next command will turn the North LED on
(gdb) next
19              *(GPIOE_BSRR as *mut u32) = 1 << 11;

(gdb) x 0x48001018
0x48001018:     0x00000000
```

Reading the register returns 0. That matches what the documentation says.

The documentation says that bits 0 to 15 can be used to set the corresponding pin. That is bit 0 sets the pin 0. Here set means outputting a high value on the pin.

The documentation also says that bits 16 to 31 can be used to reset the corresponding pin. In this case, the bit 16 resets the pin number 0. As you may guess, reset means outputting a low value on the pin.

Correlating with our program:

* Writing 1 << 9 (BS9 = 1) to BSRR sets PE9 high. That turns the North LED on.
* Writing 1 << 11 (BS11 = 1) to BSRR sets PE11 high. That turns the East LED on.
* Writing 1 << 25 (BR9 = 1) to BSRR sets PE9 low. That turns the North LED off.
* Finally, writing 1 << 27 (BR11 = 1) to BSRR sets PE11 low. That turns the East LED off.

## (mis)Optimization

In the previous example we wrote four different values to the same register.

The LLVM does not know that we are dealing with a register and will merge the writes thus changing the behavior of our program.

```
$ cargo run --release
(..)
Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:7
7       #[entry]

(gdb) step
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:9
9           aux7::init();

(gdb) next
25              *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);

(gdb) disassemble /m
Dump of assembler code for function _ZN9registers18__cortex_m_rt_main17h45b1ef53e18aa8d0E:
8       fn main() -> ! {
   0x08000248 <+0>:     push    {r7, lr}
   0x0800024a <+2>:     mov     r7, sp

9           aux7::init();
   0x0800024c <+4>:     bl      0x8000260 <aux7::init>
   0x08000250 <+8>:     movw    r0, #4120       ; 0x1018
   0x08000254 <+12>:    mov.w   r1, #134217728  ; 0x8000000
   0x08000258 <+16>:    movt    r0, #18432      ; 0x4800

10
11          unsafe {
12              // A magic address!
13              const GPIOE_BSRR: u32 = 0x48001018;
14
15              // Turn on the "North" LED (red)
16              *(GPIOE_BSRR as *mut u32) = 1 << 9;
17
18              // Turn on the "East" LED (green)
19              *(GPIOE_BSRR as *mut u32) = 1 << 11;
20
21              // Turn off the "North" LED
22              *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
23
24              // Turn off the "East" LED
25              *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
=> 0x0800025c <+20>:    str     r1, [r0, #0]
   0x0800025e <+22>:    b.n     0x800025e <registers::__cortex_m_rt_main+22>

End of assembler dump.
```

The state of the LEDS didn't change this time! the `str` instruction is the one that writes a value to the register. Our debug (unoptimized) program had four of them, one for each write to the register, but the release (optimized) program only has one.

Checking using objdump and capture the output to out.asm
```
# same as cargo objdump -- -d --no-show-raw-insn --print-imm-hex --source target/thumbv7em-none-eabihf/debug/registers
cargo objdump --bin registers -- -d --no-show-raw-insn --print-imm-hex --source > debug.txt
```

```
080001ec <main>:
; #[entry]
 80001ec:       push    {r7, lr}
 80001ee:       mov     r7, sp
 80001f0:       bl      #0x2
 80001f4:       trap

080001f6 <registers::__cortex_m_rt_main::hc2e3436fa38cd6f2>:
; fn main() -> ! {
 80001f6:       push    {r7, lr}
 80001f8:       mov     r7, sp
;     aux7::init();
 80001fa:       bl      #0x3e
 80001fe:       b       #-0x2 <registers::__cortex_m_rt_main::hc2e3436fa38cd6f2+0xa>
;         *(GPIOE_BSRR as *mut u32) = 1 << 9;
 8000200:       movw    r0, #0x2640
 8000204:       movt    r0, #0x800
 8000208:       ldr     r0, [r0]
 800020a:       movw    r1, #0x1018
 800020e:       movt    r1, #0x4800
 8000212:       str     r0, [r1]
;         *(GPIOE_BSRR as *mut u32) = 1 << 11;
 8000214:       movw    r0, #0x2648
 8000218:       movt    r0, #0x800
 800021c:       ldr     r0, [r0]
 800021e:       str     r0, [r1]
;         *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);
 8000220:       movw    r0, #0x2650
 8000224:       movt    r0, #0x800
 8000228:       ldr     r0, [r0]
 800022a:       str     r0, [r1]
;         *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
 800022c:       movw    r0, #0x2638
 8000230:       movt    r0, #0x800
 8000234:       ldr     r0, [r0]
 8000236:       str     r0, [r1]
;     loop {}
 8000238:       b       #-0x2 <registers::__cortex_m_rt_main::hc2e3436fa38cd6f2+0x44>
 800023a:       b       #-0x4 <registers::__cortex_m_rt_main::hc2e3436fa38cd6f2+0x44>
 (..)
```

To keep the LLVM from misoptimizing our program we need to use volatile operatrions instead of plan reads/writes

after updating [main.rs](../registers/src/main.rs) to use volatile writes generate release.tx with --release mode
```
cargo objdump --release --bin registers -- -d --no-show-raw-insn --print-imm-hex --source > release.txt
```

Now find the main routine in release.txt and we see the 4 `str` instructions.
```
0800023e <main>:
; #[entry]
 800023e:       push    {r7, lr}
 8000240:       mov     r7, sp
 8000242:       bl      #0x2
 8000246:       trap

08000248 <registers::__cortex_m_rt_main::h45b1ef53e18aa8d0>:
; fn main() -> ! {
 8000248:       push    {r7, lr}
 800024a:       mov     r7, sp
;     aux7::init();
 800024c:       bl      #0x22
 8000250:       movw    r0, #0x1018
 8000254:       mov.w   r1, #0x200
 8000258:       movt    r0, #0x4800
;         intrinsics::volatile_store(dst, src);
 800025c:       str     r1, [r0]
 800025e:       mov.w   r1, #0x800
 8000262:       str     r1, [r0]
 8000264:       mov.w   r1, #0x2000000
 8000268:       str     r1, [r0]
 800026a:       mov.w   r1, #0x8000000
 800026e:       str     r1, [r0]
 8000270:       b       #-0x4 <registers::__cortex_m_rt_main::h45b1ef53e18aa8d0+0x28>
 (..)
```

We also see the expected behavior using gdb
```
0800023e <main>:
; #[entry]
 800023e:       push    {r7, lr}
 8000240:       mov     r7, sp
 8000242:       bl      #0x2
 8000246:       trap

08000248 <registers::__cortex_m_rt_main::h45b1ef53e18aa8d0>:
; fn main() -> ! {
 8000248:       push    {r7, lr}
 800024a:       mov     r7, sp
;     aux7::init();
 800024c:       bl      #0x22
 8000250:       movw    r0, #0x1018
 8000254:       mov.w   r1, #0x200
 8000258:       movt    r0, #0x4800
;         intrinsics::volatile_store(dst, src);
 800025c:       str     r1, [r0]
 800025e:       mov.w   r1, #0x800
 8000262:       str     r1, [r0]
 8000264:       mov.w   r1, #0x2000000
 8000268:       str     r1, [r0]
 800026a:       mov.w   r1, #0x8000000
 800026e:       str     r1, [r0]
 8000270:       b       #-0x4 <registers::__cortex_m_rt_main::h45b1ef53e18aa8d0+0x28>
 (..)
```