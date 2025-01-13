use crate::print;
use crate::string::String;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
}

pub fn key_handle(c: char) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        match stdin.as_str() {
            "help" => {
                print!("Lightweight easy to use operating system made to limit e-waste");
            }
            _ => {
                print!("?");
            }
        }
        stdin.clear();
        print!("\n> ");
    } else {
        if c == 0x08 as char {
            if stdin.len() > 0 {
                stdin.pop();
                print!("{}", c);
            }
        } else {
            stdin.push(c as u8);
            print!("{}", c);
        }
    }
}
