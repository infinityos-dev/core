#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(infinity_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use infinity_os::kernel;
use infinity_os::print;
use infinity_os::user::shell;
use x86_64::{structures::paging::Page, VirtAddr};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    infinity_os::init();

    shell::print_banner();

    /* Memory Initialization */
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { kernel::memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    print!("heap_value at {:p}\n", heap_value);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    print!("vec at {:p}\n", vec.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    print!("current reference count is {}\n", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    print!("reference count is {} now\n", Rc::strong_count(&cloned_reference));

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    kernel::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    shell::print_prompt();

    infinity_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    infinity_os::hlt_loop();
}
