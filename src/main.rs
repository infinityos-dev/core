#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(infinity_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use infinity_os::kernel;
use infinity_os::print;
use infinity_os::user::shell;
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    infinity_os::init();

    shell::print_banner();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { kernel::memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        print!("{:?} -> {:?}\n", virt, phys);
    }

    shell::print_prompt();

    infinity_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}\n", info);
    infinity_os::hlt_loop();
}
