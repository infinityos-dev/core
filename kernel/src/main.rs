#![no_std]
#![no_main]

use core::arch::asm;
use flanterm::sys::flanterm_context;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker, BootloaderInfoRequest};
use limine::BaseRevision;
use infinity_os::utils::option_to_c_void;
use core::ptr::null_mut;
use core::ptr;

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".limine_requests_end")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static INFO_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    infinity_os::init();
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {

            let flanterm_ctx: *mut flanterm::sys::flanterm_context = flanterm::sys::flanterm_fb_init(
                None,
                None,
                framebuffer.addr() as *mut u32, framebuffer.width() as usize, framebuffer.height() as usize, framebuffer.pitch() as usize,
                framebuffer.red_mask_size(), framebuffer.red_mask_shift(),
                framebuffer.green_mask_size(), framebuffer.green_mask_shift(),
                framebuffer.blue_mask_size(), framebuffer.blue_mask_shift(),
                ptr::null_mut(),
                ptr::null_mut(), ptr::null_mut(),
                ptr::null_mut(), ptr::null_mut(),
                ptr::null_mut(), ptr::null_mut(),
                option_to_c_void::<fn()>(None), 0, 0, 1,
                None::<fn()>.is_some() as usize, None::<fn()>.is_some() as usize,
                None::<fn()>.is_some() as usize);

                let hello_msg = b"Hello!\0";
                flanterm::sys::flanterm_write(
                    flanterm_ctx, 
                    hello_msg.as_ptr() as *const i8,
                    hello_msg.len() - 1
                );
        }
    }

    hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}
