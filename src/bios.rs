use emulator::Emulator;
use register::*;
use io;

static BIOS_TO_TERMINAL: &'static [i32; 8] = &[38, 34, 32, 36, 31, 35, 33, 37];

pub fn video(emu: &mut Emulator) {
    let ah = emu.get_register8(AH);
    match ah {
        0x0e => video_teltype(emu),
        _ => unimplemented!(),

    }
}

pub fn video_teltype(emu: &mut Emulator) {
    let color = emu.get_register8(BL) & 0x0f;
    let ch = emu.get_register8(AL);
    let t_color = BIOS_TO_TERMINAL[(color & 0x07) as usize];
    let bright = if color & 0x08 != 0x0 { 1 } else { 0 };

    put_string(format!("\x1b[{:?};{:?}m{}\x1b[0m", t_color, bright, ch as char));
}


fn put_string(s: String) {
    for v in s.as_bytes() {
        io::io_out8(0x03f8, v.to_owned())
    }
}
