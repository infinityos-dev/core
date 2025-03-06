use core::ptr;
use core::ffi::c_void;

#[cfg(target_pointer_width = "64")]
pub fn u64_to_usize(value: u64) -> usize {
    value as usize
}

#[cfg(target_pointer_width = "32")]
pub fn u64_to_usize(value: u64) -> Option<usize> {
    if value <= usize::MAX as u64 {
        Some(value as usize)
    } else {
        None
    }
}

pub fn option_to_c_void<T>(opt: Option<&mut T>) -> *mut c_void {
    opt.map_or(ptr::null_mut(), |reference| reference as *mut T as *mut c_void)
}
