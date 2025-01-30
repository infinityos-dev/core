#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(infinity_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use infinity_os::kernel;
use infinity_os::kernel::task::keyboard;
use infinity_os::kernel::task::{executor::Executor, Task};
use infinity_os::print;
use x86_64::VirtAddr;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    infinity_os::init();

    /* Memory Initialization */
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { kernel::memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let args: [&str; 2] = ["date", "--metric"];
    kernel::date::main(&args);

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::keyboard_task()));
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    kernel::debug::stack_trace::walk_stack();
    infinity_os::hlt_loop();
}
