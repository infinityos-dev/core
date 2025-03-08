#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;
use infinity_os::utils::option_to_c_void;
use limine::request::{
    BootloaderInfoRequest, FramebufferRequest, RequestsEndMarker, RequestsStartMarker,
};
use limine::BaseRevision;

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

static mut FLANTERM_CTX: *mut flanterm::sys::flanterm_context = ptr::null_mut();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    infinity_os::init();
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            FLANTERM_CTX = flanterm::sys::flanterm_fb_init(
                None,
                None,
                framebuffer.addr() as *mut u32,
                framebuffer.width() as usize,
                framebuffer.height() as usize,
                framebuffer.pitch() as usize,
                framebuffer.red_mask_size(),
                framebuffer.red_mask_shift(),
                framebuffer.green_mask_size(),
                framebuffer.green_mask_shift(),
                framebuffer.blue_mask_size(),
                framebuffer.blue_mask_shift(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                option_to_c_void::<fn()>(None),
                0,
                0,
                1,
                None::<fn()>.is_some() as usize,
                None::<fn()>.is_some() as usize,
                None::<fn()>.is_some() as usize,
            );

            flanterm::sys::flanterm_write(
                FLANTERM_CTX,
                "Hello!".as_ptr() as *const i8,
                "Hello!".len(),
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
