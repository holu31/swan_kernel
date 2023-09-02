use swan_kernel::{task::keyboard, println, print};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::stream::StreamExt;
use alloc::{
    string::String,
    string::ToString,
    vec::Vec,
    format
};
use hashbrown::HashMap;
use swan_kernel::arch::x86_64::vga_buffer::*;

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
                        match character {
                            '\n' => { // Enter
                                print!("{}", character);
                                if buffer.len() > 0 {
                                    parse(buffer.clone());
                                    buffer.clear();
                                }
                                
                                WRITER.lock().write_custom_byte(CONFIG.0, CONFIG.1);
                            },
                            '\u{8}' => { // Backspace
                                if buffer.len() > 0 {
                                    buffer.pop();
                                    WRITER.lock().pop(1);
                                }
                            },
                            _ => {
                                print!("{}", character);
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

    pub fn build(
        &self, 
        buffer: String,
    ) -> Result<HashMap<String, String>, String> {
        let mut args = buffer.split_whitespace().collect::<Vec<&str>>();
        let command = args.remove(0).to_string();

        if command == self.command {

            if self.args.len() != args.len() {
                return Err(format!("args != {}", self.args.len()));
            }

            let mut parsed_args = HashMap::new();

            for arg in 0..self.args.len() {
                parsed_args.insert(
                    self.args[arg].clone(),
                    args[arg].to_string()
                );
            }

            return Ok(parsed_args);
        }
        Err(format!("not found '{}' command", buffer))
    }

}

fn parse(buffer: String) {

    // TODO: Make a search command

    match Command::new("test")
        .arg("test_arg")
        .build(buffer.clone()) {
            
            Ok(args) => println!("{}", args["test_arg"]),
            Err(_) => {}
    };

    match Command::new("help")
        .build(buffer.clone()) {
            
            Ok(_) => println!("help"),
            Err(_) => {}
    };
}
