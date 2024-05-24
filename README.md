powerswitch
===========

Rust project using the _Arduino Uno_ as a power switch for a PC.

## Setup

Connect PIN8 to PWR+ and GND to PWR- on your motherboard.
send button signal via `exec 3<> /dev/ttyACM0 && stty -F /dev/ttyACM0 57600 && sleep 5 && echo 'BTN:ON' > /dev/ttyACM0`



## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

