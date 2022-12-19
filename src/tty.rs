#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray
}

pub struct Writer {
    buffer: *mut [[(u8, Color); 80]; 25],
    pub color: Color,
    cursor: (usize, usize)
}

impl Writer {
    pub fn init() -> Self {
        Self {
            color: Color::LightGray,
            cursor: (0, 0),
            buffer: 0xB8000 as *mut _
        }
    }

    pub fn cursor_xy(&mut self, x: usize, y: usize){
        self.cursor = (x, y);
    }

    pub fn cursor_move(&mut self, offset: usize){
        self.cursor.0 += offset;
    }

    pub fn write_byte(&mut self, byte: u8){
        unsafe {
            if byte == 0 || self.cursor.1 > 25 {
                return;
            } else if byte == b'\n' || self.cursor.0 == 80 {
                self.cursor.0 = 0;
                self.cursor.1 += 1;
                return;
            }
            (&mut *self.buffer)[self.cursor.1][self.cursor.0] = (byte, self.color.clone());
            self.cursor_move(1);
        }
    }

    pub fn write_string(&mut self, string: &str){
        string.bytes()
            .for_each(|i| self.write_byte(i));
    }
}

