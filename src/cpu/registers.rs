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
    pub f: Flags
}

pub enum Reg8 {
    A
}

pub enum Reg16 {
    PC
}
