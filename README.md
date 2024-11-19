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