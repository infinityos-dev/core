use core::fmt;
use core::ptr;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::serial_println;

#[allow(dead_code)]
pub struct FlantermContextWrapper(*mut flanterm::sys::flanterm_context);

impl FlantermContextWrapper {
    pub fn new(context: *mut flanterm::sys::flanterm_context) -> Self {
        FlantermContextWrapper(context)
    }

    pub fn inner(&self) -> *mut flanterm::sys::flanterm_context {
        self.0
    }
}

unsafe impl Send for FlantermContextWrapper {}
unsafe impl Sync for FlantermContextWrapper {}

lazy_static! {
    pub static ref FLANTERM_CTX: Mutex<FlantermContextWrapper> =
        Mutex::new(FlantermContextWrapper(ptr::null_mut()));
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    });
}

pub struct Writer {
}

impl Writer {
    fn write_string(&mut self, s: &str) {
        serial_println!("{}", s);
        unsafe { 
            flanterm::sys::flanterm_write(
                FLANTERM_CTX.lock().inner(), 
                s.as_ptr() as *const i8,
                s.len()
            ) 
        };
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::writer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
