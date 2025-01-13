#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(infinity_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use infinity_os::{print, shell};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    infinity_os::init();

    shell::print_banner();
    shell::print_prompt();

    infinity_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    infinity_os::hlt_loop();
}
