use std::fmt;

bitflags!(
    pub struct Flags: u8 {
        const Z = 0b1000000;
        const N = 0b0100000;
        const H = 0b0010000;
        const C = 0b0001000;
    }
);

impl Flags {
    pub fn test(&self, test: bool) -> Flags {
        if test {
            *self
        } else {
            Flags::empty()
        }
    }
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub pc: u16,
    pub sp: u16,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC:0x{:04x} SP:0x{:04x} \
               A:0x{:02x} B:0x{:02x} C:0x{:02x} D:0x{:02x} \
               E:0x{:02x} H:0x{:02x} L:0x{:02x} F:{:?}",
               self.pc, self.sp,
               self.a, self.b, self.c, self.d,
               self.e, self.h, self.l, self.f)
    }
}

#[derive(Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum Reg16 {
    PC,
    HL,
    AF,
    BC,
    DE,
    SP,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            pc: 0x0100,
            sp: 0xfffe,
            f: Z|H|C,
            h: 0x01,
            l: 0x4d,
        }
    }
}
