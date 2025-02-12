use crate::print;
use crate::kernel::string::String;
use lazy_static::lazy_static;
use spin::Mutex;
use super::super::kernel;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
}

pub fn key_handle(c: char) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        match stdin.as_str() {
            "help" => {
                print!("InfinityOS is a lightweight easy to use operating system made to limit e-waste");
            },
            "uptime" => {
                print!("{:.1} seconds\n", kernel::clock::uptime());
            },
            _ => {
                print!("Unknown command");
            }
        }
        stdin.clear();
        print!("\n> ");
    } else {
        if c != '\\' {
            stdin.push(c as u8);
            print!("{}", c);
        }
    }
}
