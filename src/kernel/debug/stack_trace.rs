use crate::print;
use core::arch::asm;

#[repr(C)]
struct StackFrame {
    rbp: *const StackFrame,
    rip: usize,
}

pub fn walk_stack() {
    let mut frame: *const StackFrame;
    unsafe {
        asm!("mov {}, rbp", out(reg) frame);
    }

    while let Some(stack_frame) = unsafe { frame.as_ref() } {
        print!("Function address: {:#x}\n", stack_frame.rip);
        frame = stack_frame.rbp;
    }
}
