use std::io;
use std::str;

pub fn io_in8(addr: u16) -> u8 {
    match addr {
        0x03f8 => {
            let mut buf = String::new();
            match io::stdin().read_line(&mut buf) {
                Ok(_) => buf.as_bytes()[0],
                _ => panic!("mis"),
            }
        }
        _ => 0,
    }
}

pub fn io_out8(addr: u16, value: u8) {
    match addr {
        0x03f8 => print!("{}", value as char),
        _ => panic!("unknown"),
    }
}
