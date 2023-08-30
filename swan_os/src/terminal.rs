use swan_kernel::{task::keyboard, println, print};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::stream::StreamExt;
use alloc::string::String;
use crate::alloc::string::ToString;
use alloc::vec::Vec;
use hashbrown::HashMap;
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

pub struct Command {
    command: String,
    args: Vec<String>
}

impl Command {

    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new()
        }
    }

    pub fn arg(&mut self, arg: &str) -> &Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn build(&self, buffer: String, f: fn(HashMap<String, String>)) {
        let mut args = buffer.split_whitespace().collect::<Vec<&str>>();
        let command = args.remove(0).to_string();

        if command == self.command {

            assert_eq!(self.args.len(),args.len());

            let mut parsed_args = HashMap::new();

            for arg in 0..self.args.len() {
                parsed_args.insert(
                    self.args[arg].clone(),
                    args[arg].to_string()
                );
            }

            f(parsed_args);
        }
    }

}

fn parse(buffer: String) {
    Command::new("test")
        .arg("test_arg")
        .build(buffer.clone(), |args| println!("{}", args["test_arg"]));

    Command::new("help")
        .build(buffer.clone(), |args| println!("kek2"));
}
