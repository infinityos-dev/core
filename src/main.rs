#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
