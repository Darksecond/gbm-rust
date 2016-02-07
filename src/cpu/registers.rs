bitflags! {
    flags Flags: u8 {
        const Z = 0b1000000,
        const N = 0b0100000,
        const H = 0b0010000,
        const C = 0b0001000
    }
}

impl Flags {
    pub fn test(&self, test: bool) -> Flags {
        if test {
            *self
        } else {
            Flags::empty()
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub pc: u16,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

pub enum Reg8 {
    A
}

pub enum Reg16 {
    PC,
    HL
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            pc: 0x0100,
            f: Flags::empty(),
            h: 0x01,
            l: 0x4d,
        }
    }
}
