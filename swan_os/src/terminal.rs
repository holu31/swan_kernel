use swan_kernel::{task::keyboard, println, print};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::stream::StreamExt;
use alloc::string::String;
use swan_kernel::vga_buffer::*;

const CONFIG: (u8, Color) = (b'>', Color::Yellow);

pub async fn run() {
    let mut scancodes = keyboard::ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore);
    let mut buffer: String = String::new();
 
    WRITER.lock().write_custom_byte(CONFIG.0, CONFIG.1);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);
                        match character {
                            '\n' => {
                                parse(buffer.clone());
                                buffer.clear();

                                WRITER.lock().write_custom_byte(CONFIG.0, CONFIG.1);
                            }
                            _ => {
                                buffer.push(character);
                            }
                        }
                    },
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}

fn parse(buffer: String) {
    match buffer.as_str() {
        "test" => println!("test command"),
        _ => println!("command '{}' not found.", buffer)
    }
}
