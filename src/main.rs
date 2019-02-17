#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    loop {}
}
