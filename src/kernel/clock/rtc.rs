use core::fmt;
use crate::print;

const RTC_ADDRESS_PORT: u16 = 0x70;
const RTC_DATA_PORT: u16 = 0x71;

const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;

/// Structure to hold the date and time
#[derive(Debug)]
pub struct DateTime {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u8,
}

/// Implement Display trait for DateTime to format it nicely
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}/{:02}/{:02} {:02}:{:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute, self.second
        )
    }
}

/// Function to read a byte from the RTC
unsafe fn rtc_read(register: u8) -> u8 {
    x86::io::outb(RTC_ADDRESS_PORT, register);
    x86::io::inb(RTC_DATA_PORT)
}

/// Function to convert BCD to binary
fn bcd_to_bin(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

/// Function to read the current date and time from the RTC
pub unsafe fn read_rtc() -> DateTime {
    let second = bcd_to_bin(rtc_read(RTC_SECONDS));
    let minute = bcd_to_bin(rtc_read(RTC_MINUTES));
    let hour = bcd_to_bin(rtc_read(RTC_HOURS));
    let day = bcd_to_bin(rtc_read(RTC_DAY));
    let month = bcd_to_bin(rtc_read(RTC_MONTH));
    let year = bcd_to_bin(rtc_read(RTC_YEAR));

    DateTime {
        second,
        minute,
        hour,
        day,
        month,
        year,
    }
}

pub fn print_date_time() {
    let date_time = unsafe { read_rtc() };
    print!("Current Date and Time: {}", date_time);
}