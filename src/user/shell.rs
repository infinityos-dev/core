use super::super::kernel;
use crate::kernel::string::String;
use crate::print;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
}

pub fn print_banner() {
    print!("                     ____\n");
    print!("                    /|o  |            ()()\n");
    print!("                   /o|  o|           (o.o )\n");
    print!("+---------------- /o_|_o_|------------> ^ <------------------------------------+\n");
    print!("|                                                                              |\n");
    print!("|                                 Infinity OS                                  |\n");
    print!("|                                                                              |\n");
    print!("|                         Lightweight Operating System                         |\n");
    print!("|                                                                              |\n");
    print!("+------------------------------------------------------------------------------+\n");
    print!("\n");
}

pub fn print_prompt() {
    print!("\n> ");
}

pub fn key_handle(c: char) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        match stdin.as_str() {
            "" => {}
            "help" => {
                print!("InfinityOS is a lightweight easy to use operating system made to limit e-waste\n");
            }
            "uptime" => {
                print!("{:.1} seconds\n", kernel::clock::uptime());
            }
            _ => {
                print!("Unknown command\n");
            }
        }
        stdin.clear();
        print_prompt();
    } else {
        if c != '\\' {
            stdin.push(c as u8);
            print!("{}", c);
        }
    }
}
