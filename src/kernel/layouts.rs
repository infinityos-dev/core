//! A standard United States 101-key (or 104-key including Windows keys) QWERTY keyboard.
//! Has a 1-row high Enter key, with Backslash above.

use pc_keyboard::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

pub struct Qwerty104Key;

impl KeyboardLayout for Qwerty104Key {
    fn map_keycode(
        keycode: KeyCode,
        modifiers: &Modifiers,
        _handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            KeyCode::BackTick => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('`')
                }
            }
            KeyCode::Key1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('!')
                } else {
                    DecodedKey::Unicode('1')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('$')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Key5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('5')
                }
            }
            KeyCode::Key6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('6')
                }
            }
            KeyCode::Key7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::Minus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Equals => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
            KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
            KeyCode::Q => DecodedKey::Unicode(if modifiers.is_shifted() { 'Q' } else { 'q' }),
            KeyCode::W => DecodedKey::Unicode(if modifiers.is_shifted() { 'W' } else { 'w' }),
            KeyCode::E => DecodedKey::Unicode(if modifiers.is_shifted() { 'E' } else { 'e' }),
            KeyCode::R => DecodedKey::Unicode(if modifiers.is_shifted() { 'R' } else { 'r' }),
            KeyCode::T => DecodedKey::Unicode(if modifiers.is_shifted() { 'T' } else { 't' }),
            KeyCode::Y => DecodedKey::Unicode(if modifiers.is_shifted() { 'Y' } else { 'y' }),
            KeyCode::U => DecodedKey::Unicode(if modifiers.is_shifted() { 'U' } else { 'u' }),
            KeyCode::I => DecodedKey::Unicode(if modifiers.is_shifted() { 'I' } else { 'i' }),
            KeyCode::O => DecodedKey::Unicode(if modifiers.is_shifted() { 'O' } else { 'o' }),
            KeyCode::P => DecodedKey::Unicode(if modifiers.is_shifted() { 'P' } else { 'p' }),
            KeyCode::BracketSquareLeft => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('[')
                }
            }
            KeyCode::BracketSquareRight => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode(']')
                }
            }
            KeyCode::BackSlash => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('\\')
                }
            }
            KeyCode::A => DecodedKey::Unicode(if modifiers.is_shifted() { 'A' } else { 'a' }),
            KeyCode::S => DecodedKey::Unicode(if modifiers.is_shifted() { 'S' } else { 's' }),
            KeyCode::D => DecodedKey::Unicode(if modifiers.is_shifted() { 'D' } else { 'd' }),
            KeyCode::F => DecodedKey::Unicode(if modifiers.is_shifted() { 'F' } else { 'f' }),
            KeyCode::G => DecodedKey::Unicode(if modifiers.is_shifted() { 'G' } else { 'g' }),
            KeyCode::H => DecodedKey::Unicode(if modifiers.is_shifted() { 'H' } else { 'h' }),
            KeyCode::J => DecodedKey::Unicode(if modifiers.is_shifted() { 'J' } else { 'j' }),
            KeyCode::K => DecodedKey::Unicode(if modifiers.is_shifted() { 'K' } else { 'k' }),
            KeyCode::L => DecodedKey::Unicode(if modifiers.is_shifted() { 'L' } else { 'l' }),
            KeyCode::SemiColon => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::Quote => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::Enter => DecodedKey::Unicode(10.into()),
            KeyCode::Z => DecodedKey::Unicode(if modifiers.is_shifted() { 'Z' } else { 'z' }),
            KeyCode::X => DecodedKey::Unicode(if modifiers.is_shifted() { 'X' } else { 'x' }),
            KeyCode::C => DecodedKey::Unicode(if modifiers.is_shifted() { 'C' } else { 'c' }),
            KeyCode::V => DecodedKey::Unicode(if modifiers.is_shifted() { 'V' } else { 'v' }),
            KeyCode::B => DecodedKey::Unicode(if modifiers.is_shifted() { 'B' } else { 'b' }),
            KeyCode::N => DecodedKey::Unicode(if modifiers.is_shifted() { 'N' } else { 'n' }),
            KeyCode::M => DecodedKey::Unicode(if modifiers.is_shifted() { 'M' } else { 'm' }),
            KeyCode::Comma => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('<')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::Fullstop => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::Slash => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('/')
                }
            }
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            _ => DecodedKey::RawKey(keycode),
        }
    }
}
