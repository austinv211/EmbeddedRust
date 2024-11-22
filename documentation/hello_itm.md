# Hello ITM
The solder bridge SB10 is unconnected by default and needs to be soldered to use ITM and the iprintln macros.

in main we are simply calling the iprinln! macro from [auxiliary/src/libr.rs](../hello_world/auxiliary/src/libr.rs)

1. solder the SB10 solder bridge
2. run openocd as specified in the previous section
3. run itm in the same folder as openocd is running
4. use cargo run to load the program, you should see "Hello World" after extering `next` in gcb past the iprintln! macro

Feel free to use iprintln as a logging tool in the upcoming sections