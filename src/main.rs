#![no_std]
#![no_main]

mod vga_buffer;


use core::panic::PanicInfo;
use core::fmt::Write;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("Hello Again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {}, {}", 42, 1.337).unwrap();

    loop {}
}