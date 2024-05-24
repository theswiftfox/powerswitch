#![no_std]
#![no_main]

use panic_halt as _;

const MAGIC_BYTES: [u8; 6] = [0x42, 0x54, 0x4E, 0x3A, 0x4F, 0x4E]; // BTN:ON

// GND -> Power-
// D13 -> Power+

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut input = pins.d8.into_floating_input();

    let mut read_line = || {
        let mut buf = [0; 10];
        let mut i = 0;
        loop {
            if i >= buf.len() {
                break;
            }
            let b = serial.read_byte();
            if b == b'\r' || b == b'\n' {
                break;
            }
            buf[i] = b;
            i += 1;
        }
        buf
    };

    loop {
        let line = read_line();

        // expect BTN:ON on serial console to trigger button
        if line[0..6] == MAGIC_BYTES {
            let mut output = input.into_output();
            output.set_low();
            arduino_hal::delay_ms(500);
            input = output.into_floating_input();
        }
    }
}
