const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut position: isize = 0;

pub fn print(s: &str){
    s.as_bytes()
        .iter()
        .flat_map(|bt| [*bt, 0x7 as u8])
        .enumerate()
        .for_each(|(i, byte)| unsafe {
            *VGA_BUFFER.offset(position) = byte;
            position += 1;
        });
}
