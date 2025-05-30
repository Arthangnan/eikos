#![allow(dead_code)]
#![no_std]
#![no_main]

mod port;
mod vga_buffer;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn teleia() {
    println!("42");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
