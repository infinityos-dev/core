#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(infinity_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use infinity_os::print;
use infinity_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    infinity_os::init();

    #[cfg(test)]
    test_main();

    println!("Welcome to Infinity OS!");
    print!("> ");

    infinity_os::arch::interrupts::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    infinity_os::arch::interrupts::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    infinity_os::test_panic_handler(info)
}

fn print_logo() {
    println!("");
    println!("  _____        __ _       _ _          ____   _____ ");
    println!(" |_   _|      / _(_)     (_) |        / __ \\ / ____|");
    println!("   | |  _ __ | |_ _ _ __  _| |_ _   _| |  | | (___  ");
    println!("   | | | '_ \\|  _| | '_ \\| | __| | | | |  | |\\___ \\ ");
    println!("  _| |_| | | | | | | | | | | |_| |_| | |__| |____) |");
    println!(" |_____|_| |_|_| |_|_| |_|_|\\__|\\__, |\\____/|_____/ ");
    println!("                                 __/ |              ");
    println!("                                |___/               ");
    println!("");
}
