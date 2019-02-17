#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga;

// static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_something();
    loop {}
}
