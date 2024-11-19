# STM32 on Rust
This is a learning project using Rust against an STM32 Discovery board.

For this repo I am following the f3discovery guide found [here](https://docs.rust-embedded.org/discovery/f3discovery/index.html)

## Getting Started
After following the guide for setting up the development environment for windows, we want to verify the installation

1. connect the STM32F3DISCOVERY to your computer using an USB cable in the USB ST-LINK port
2. Verifying connection with OpenOCD
```
openocd -s C:\OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```
OpenOCD is a service which forwards debug information from the ITM channel to a file, itm.txt, as such it runs forever and does not return to the terminal prompt.
You'll notice one of the Red LEDs, closest to the USB port, should start oscillating between red light and green light.

## STM32F3DISCOVERY (the "F3")
Here are some of the many components on the board
* a microcontroller
* a number of LEDs, including the eight aligned in a "compass" formation
* Two buttons
* Two USB ports
* An accelerometer
* A magnetometer
* A gyroscape

The MCU is what runs your code

Our MCU is surrounded by 100 tiny metal pins. These pins are connected to traces, the little "roads" that act as the wires connecting components together on the board
The MCU can dynamically alter the electrical properties of the pins. This works similar to a light switch altering how electrical current flows through a circuit.
By enabling or disabling electrical current to flow through a specific pin, an LED attached to that pin (via the traces) can be turned on and off.

Each manufacturer uses a different part numbering scheme, but many will allow you to determine information about a component simply by looking at the part number.
Our MCU's part number is STM32F303VCT6

* the M32 reperesnets that this is an Arm-based 32-bit microcontroller
* the F3 reperesents that the MCU is from ST's "STM32F3" series. This is a series of MCU's based on the Cortex-M4 processor design
* The remainder of the part number goes into more details about things like extra features and RAM size

## LED roulette
For now we are using a high-level API to implement this app, this is mainly to get familiar with the flashing and debugging process.

We are going to create a new project for our LED roulette
```
cargo new led-roulette
```

This app should use no_std and no_main since we aren't running this against an OS. See [freestanding_binary.md][https://github.com/austinv211/RustOS/blob/master/documentation/freestanding_binary.md] for my previous notes on no_std

## Building It
Specify the correct target in config.toml so we can just run cargo build (Note: I also add vscode target for linting purposes)

after running cargo build, let's verify that we actually have an ARM binary

## Flash it
Flashing is the process of moving our program into the microcontroller's persistent memory.

For Windows
```
openocd -s C:\openOCD\share\scripts -f interface/stlink.cfg -f target/stm32f3x.cfg
```

this program will block; leave that terminal open.

what the openocd command is actually doing.
The F3 actually has two microcontrollers. One of them is used as a programmer/debugger. The part of the board that's used as a programmer is called ST-LINK. this ST-LINK is connected to the target microcontroller using a Serial Wire Debug (SWD) interface.
SWD is an ARM standard so you'll run into it when dealing with other Cortex-M based microcontrollers. This SWD interface can be used to flash and debug a microcontroller.

As for OpenOCD, it's software that provides some services like a GDB server on top of USC devices that expose a debugging protocol like SWD or JTAG.

We are expecting output similar to this
```
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
    http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v37 API v2 SWIM v26 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.888183
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

The "6 breakpoints , 4 watchpoints" part indicates the debugging features the processor has available.
Let's connect to the GDB server

## Execute GDB
first, we need to determine what version of gdb you have that is capable of debugging ARM binaries.
```
arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette
```

we will specify this in our config.toml so that cargo run puts us into gdb

when in gdb we can load our program with `load`

## Debug it
After the `load` command, our program is stopped at its entry point.
This is indicated by the "Start address at ..." part of GDB's output.

The starter code has some extra code that runs before the main function, at this time we are not interested in that pre-main part so let's skip to the beginnging of the main function.
We'll do that using a breakpoint. Issue `break main` at the gdb prompt. You'll see output about the breakpoint, then run `continue` to let the program run freely until it reaches a breakpoint.
In this case until it reaches #[entry] which is a trampoline to the main function and where break main sets the breakpoint. Reminder our processor can only use 6 breakpoints

since we are stopped at #[entry] and using the `disassemble /m` we see the code for entry, which is a trampoline to main. That means it sets up the stack and then invokes a subroutine call to the main function using an ARM branch and link instruction, bl.
```
(gdb) disassemble /m
Dump of assembler code for function main:
7       #[entry]
   0x080001ec <+0>:     push    {r7, lr}
   0x080001ee <+2>:     mov     r7, sp
=> 0x080001f0 <+4>:     bl      0x80001f6 <_ZN12led_roulette18__cortex_m_rt_main17he61ef18c060014a5E>
   0x080001f4 <+8>:     udf     #254    ; 0xfe

End of assembler dump.
```

next we issue a step GDB command which will advance the program statement by statement stepping into functions/procedures. After this first step command we're inside main and are positioned at the first executable rust statement (but not executed)
```
(gdb) step
led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:10
10          let x = 42;
```

next, we'll issue a second step command which executes line 10 and stops at line 11 `_y = x` (again line 11 is not executed)
```
(gdb) step
11          _y = x;
```

We are now on the `_y = x` statement that hasn't been executed yet. This means that x is initialized but _y is not. Let's inspect those values using the print command.
```
(gdb) print x
$1 = 42
(gdb) p &x
$2 = (*mut i32) 0x20009fe0
(gdb) p _y
$3 = 536870912
(gdb) p &_y
$4 = (*mut i32) 0x20009fe4
```
_y has not been initialized yet and contains some garbage value. Note we can also use `info locals` to print out the local values.
With another step we'll be on top of the `loop {}` statement

As introduced above the disassemble /m command can be used to disassemble the program around the line you are currently at. You might also want to `set print asm-demangle` so the names are demangled (only needs to be done once a debug session)
```
(gdb) set print asm-demangle on
(gdb) disassemble /m
Dump of assembler code for function _ZN12led_roulette18__cortex_m_rt_main17h51e7c3daad2af251E:
8       fn main() -> ! {
   0x080001f6 <+0>:     sub     sp, #8
   0x080001f8 <+2>:     movs    r0, #42 ; 0x2a

9           let _y;
10          let x = 42;
   0x080001fa <+4>:     str     r0, [sp, #0]

11          _y = x;
   0x080001fc <+6>:     str     r0, [sp, #4]

12
13          // infinite loop; just so we don't leave this stack frame
14          loop {}
=> 0x080001fe <+8>:     b.n     0x8000200 <led_roulette::__cortex_m_rt_main+10>
   0x08000200 <+10>:    b.n     0x8000200 <led_roulette::__cortex_m_rt_main+10>

End of assembler dump.
```
Also, as mentioned above if you were to execute the step command GDB gets stuck because it is executing a branch instruction to itself and never gets past it. So you need to use Ctrl+C to regain control. An alternative is to use the stepi(si) GDB command, which steps one asm instruction, and GDB will print the address and line number of the statement the processor will execute next and it won't get stuck.

We can use `monitor reset halt` to move back to the beginning of entry. Monitor reset halt will reset the microcontroller and stop it right at the beginning of the program. The continue command will then the let the program run freely until it reaches a breakpoint, in this case it is the breakpoint at `#[entry]`

You can end a debug session with `quit`

## The LED and Delay abstractions
Introducing two high level abstractions that we'll use to implement the LED roulette application.

The auxiliary crate aux5 exposes an intitialization function called init. When called, this function returns two values packed in a tuple: A Delay value and a LedArray value.

Delay can be used to block your program for a specified amount of milliseconds.
LedArray is an array of 8 LEDs (exposes on and off)

After modifications to main.rs and cargo build
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `arm-none-eabi-gdb -q ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette`
Reading symbols from ~/embedded-discovery/target/thumbv7em-none-eabihf/debug/led-roulette...

(gdb) target remote :3333
Remote debugging using :3333
led_roulette::__cortex_m_rt_main_trampoline () at ~/embedded-discovery/src/05-led-roulette/src/main.rs:7
7       #[entry]

(gdb) load
Loading section .vector_table, size 0x194 lma 0x8000000
Loading section .text, size 0x52c0 lma 0x8000194
Loading section .rodata, size 0xb50 lma 0x8005454
Start address 0x08000194, load size 24484
Transfer rate: 21 KB/sec, 6121 bytes/write.

(gdb) break main
Breakpoint 1 at 0x8000202: file ~/embedded-discovery/src/05-led-roulette/src/main.rs, line 7.
Note: automatically using hardware breakpoints for read-only addresses.

(gdb) continue
Continuing.

Breakpoint 1, led_roulette::__cortex_m_rt_main_trampoline ()
    at ~/embedded-discovery/src/05-led-roulette/src/main.rs:7
7       #[entry]

(gdb) step
led_roulette::__cortex_m_rt_main () at ~/embedded-discovery/src/05-led-roulette/src/main.rs:9
9           let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

(gdb)
```

```
(gdb) next
11          let half_period = 500_u16;

(gdb) next
13          loop {

(gdb) next
14              leds[0].on().ok();

(gdb) next
15              delay.delay_ms(half_period);
```

```
(gdb) next
17              leds[0].off().ok();

(gdb) next
18              delay.delay_ms(half_period);
```
after executing leds[0].on().ok(); the RED LED pointing north with turn on.
the delay_ms call will block the program for half a second but you may not notice because the next command also takes some time to execute.

a more explicit way to show the arguments of a function is to use the `info args` command.
Regardless of where your program may have stopped you can always look at the output of `backtrace` command

modify a variable with the `set` command
```
(gdb) set half_period = 100

(gdb) print half_period
$5 = 100
```

```
(gdb) break main.rs:14
Breakpoint 2 at 0x8000236: file src/05-led-roulette/src/main.rs, line 14.
(gdb) continue
Continuing.

Breakpoint 2, led_roulette::__cortex_m_rt_main () at src/05-led-roulette/src/main.rs:14
14              leds[0].on().ok();

```

the reason the delay didn't change was because the compiler recognized that half_period didn't change and instead in the two places where delay.delay_ms(half_period); is called we see mov.w r1, #500. So changing the value of half_period does nothing. One solution to the problem is to wrap half_period in a `Volatile`