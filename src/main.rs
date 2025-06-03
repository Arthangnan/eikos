#![allow(dead_code)]
#![no_std]
#![no_main]

mod port;
mod vga_buffer;

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn teleia() {
    clear_screen!();
    loop {
        if let Ok(data) = SERIAL1.lock().try_receive() {
            if data == 13 {
                println!();
            } else if data == 12 {
                clear_screen!();
            } else {
                print!("{}", data as char);
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
