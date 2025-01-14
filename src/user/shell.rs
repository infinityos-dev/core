use crate::print;
use crate::kernel::clock;
use crate::kernel::string::String;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
}

pub fn print_banner() {
    print!("                     ____\n");
    print!("                    /|o  |            ()()\n");
    print!("                   /o|  o|           (o.o )\n");
    print!("+-----------------/o_|_o_|------------> ^ <------------------------------------+\n");
    print!("|                                                                              |\n");
    print!("|                                 Infinity OS                                  |\n");
    print!("|                                                                              |\n");
    print!("|                         Lightweight Operating System                         |\n");
    print!("|                                                                              |\n");
    print!("+------------------------------------------------------------------------------+\n");
}

pub fn print_prompt() {
    print!("\n> ");
}

pub fn key_handle(c: char) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        match stdin.as_str() {
            "help" => {
                print!("Lightweight easy to use operating system made to limit e-waste");
            },
            "version" => {
                print!("Infinity OS v{}", env!("CARGO_PKG_VERSION"));
            },
            "uptime" => {
                print!("{:.0} seconds", clock::uptime());
            },
            _ => {
                print!("Unknown command: {}", stdin.as_str());
            }
        }
        stdin.clear();
        print!("\n");
        print_prompt();
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
