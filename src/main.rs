#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(infinity_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use infinity_os::user::shell;
use core::panic::PanicInfo;
use infinity_os::kernel;
use infinity_os::print;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    infinity_os::init(boot_info);

    shell::print_banner();
    shell::print_prompt();
    infinity_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    kernel::debug::stack_trace::walk_stack();
    infinity_os::hlt_loop();
}
