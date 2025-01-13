#![no_std]
#![no_main]

use core::panic::PanicInfo;
use infinity_os::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("********************************************************************************\n");
    print!("*                        Welcome to Infinity OS v0.1.0                         *\n");
    print!("********************************************************************************\n");
    print!("\n");

    infinity_os::init();

    print!("> ");
    infinity_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    infinity_os::hlt_loop();
}
