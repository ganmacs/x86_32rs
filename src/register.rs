const REGISTERS_COUNT: usize = 8;

#[allow(dead_code)]
pub const EAX: usize = 0;
#[allow(dead_code)]
pub const ECX: usize = 1;
#[allow(dead_code)]
pub const EDX: usize = 2;
#[allow(dead_code)]
pub const EBX: usize = 3;
#[allow(dead_code)]
pub const ESP: usize = 4;
#[allow(dead_code)]
pub const EBP: usize = 5;
#[allow(dead_code)]
pub const ESI: usize = 6;
#[allow(dead_code)]
pub const EDI: usize = 7;

#[allow(dead_code)]
pub const AL: usize = EAX;
#[allow(dead_code)]
pub const CL: usize = ECX;
#[allow(dead_code)]
pub const DL: usize = EDX;
#[allow(dead_code)]
pub const BL: usize = EBX;

#[allow(dead_code)]
pub const AH: usize = AL + 4;
#[allow(dead_code)]
pub const CH: usize = CL + 4;
#[allow(dead_code)]
pub const DH: usize = DL + 4;
#[allow(dead_code)]
pub const BH: usize = BL + 4;

const REGISTER_NAMES: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

pub struct Register {
    registers: [u32; REGISTERS_COUNT],
}

impl Register {
    pub fn new() -> Register {
        Register { registers: [0; REGISTERS_COUNT] }
    }

    pub fn get(&mut self, i: usize) -> u32 {
        self.registers[i]
    }

    pub fn set(&mut self, i: usize, value: u32) {
        self.registers[i] = value
    }

    pub fn dump(&mut self) {
        for i in 0..REGISTERS_COUNT {
            println!("{} {:08X}", REGISTER_NAMES[i], self.registers[i]);
        }
    }
}
