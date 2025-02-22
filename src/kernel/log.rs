use crate::kernel::vga::{Color, WRITER};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

pub fn log(level: LogLevel, message: &str) {
    let (label, fg_color, bg_color) = match level {
        LogLevel::Trace => ("TRACE", Color::LightGray, Color::Black),
        LogLevel::Debug => ("DEBUG", Color::Cyan, Color::Black),
        LogLevel::Info => ("INFO ", Color::White, Color::Black),
        LogLevel::Warn => ("WARN ", Color::Yellow, Color::Black),
        LogLevel::Error => ("ERROR", Color::Red, Color::Black),
        LogLevel::Fatal => ("FATAL", Color::Magenta, Color::Black),
        LogLevel::Panic => ("PANIC", Color::White, Color::Red), // White on red for full message
    };

    // Write directly using the writer instead of print! macro. Or else a deadlock will occur because of the second lock in the vga::_print() function.
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        use core::fmt::Write;

        if level == LogLevel::Panic {
            // Panic messages should be fully white on black
            writer.set_color(Color::White, Color::Red);
            write!(writer, "[{}]: {}", label, message).unwrap();
        } else {
            // Normal logs: color only the label
            writer.set_color(fg_color, bg_color);
            write!(writer, "[{}]: ", label).unwrap();

            // Reset color for the message
            writer.set_color(Color::White, Color::Black);
            writeln!(writer, "{}", message).unwrap();
        }
    });
}
