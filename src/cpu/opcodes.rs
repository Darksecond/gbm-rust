
use cpu::{DecInc,In8,In16,Out8,Out16, Cond};
use cpu::CPU;
use cpu::registers::{Reg8, Reg16};

#[derive(Debug)]
pub enum Op8 {
    Register(Reg8),
    Immediate(u8),
    Memory(Addr),
}

#[derive(Debug)]
pub enum Addr {
    HL, HLD, HLI,
    Immediate(u16),
}

#[derive(Debug)]
pub enum Op16 {
    Register(Reg16),
    Immediate(u16),
}

#[derive(Debug)]
pub enum Opcode {
    Unknown(u8),
    Nop,
    Dec(Op8),
    Xor(Op8),
    Jp(Cond, Op16),
    Jr(Cond, u8),
    Ld(Op8, Op8),
    Ld16(Op16, Op16),
    Rra,
}

impl Opcode {
    pub fn decode(cpu: &mut CPU) -> Opcode {
        let opcode = cpu.next_u8();
        match opcode {
            0x00 => Opcode::Nop,
            0x05 => Opcode::Dec(Op8::Register(Reg8::B)),
            0x06 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Immediate(cpu.next_u8())),
            0x0E => Opcode::Ld(Op8::Register(Reg8::C), Op8::Immediate(cpu.next_u8())),
            0x15 => Opcode::Dec(Op8::Register(Reg8::D)),
            0x1F => Opcode::Rra,
            0x20 => Opcode::Jr(Cond::NZ, cpu.next_u8()),
            0x21 => Opcode::Ld16(Op16::Register(Reg16::HL), Op16::Immediate(cpu.next_u16())),
            0x25 => Opcode::Dec(Op8::Register(Reg8::H)),
            0x32 => Opcode::Ld(Op8::Memory(Addr::HLD), Op8::Register(Reg8::A)),
            0xAF => Opcode::Xor(Op8::Register(Reg8::A)),
            0xC3 => Opcode::Jp(Cond::Always, Op16::Immediate(cpu.next_u16())),
            _ => Opcode::Unknown(opcode),
        }
    }

    pub fn opcode(&self) -> u8 {
        match *self {
            Opcode::Nop => 0x00,
            Opcode::Dec(Op8::Register(Reg8::B)) => 0x05,
            Opcode::Ld(Op8::Register(Reg8::B), Op8::Immediate(_)) => 0x06,
            Opcode::Ld(Op8::Register(Reg8::C), Op8::Immediate(_)) => 0x0E,
            Opcode::Dec(Op8::Register(Reg8::D)) => 0x15,
            Opcode::Rra => 0x1F,
            Opcode::Jr(Cond::NZ, _) => 0x20,
            Opcode::Ld16(Op16::Register(Reg16::HL), Op16::Immediate(_)) => 0x21,
            Opcode::Dec(Op8::Register(Reg8::H)) => 0x25,
            Opcode::Ld(Op8::Memory(Addr::HLD), Op8::Register(Reg8::A)) => 0x32,
            Opcode::Xor(Op8::Register(Reg8::A)) => 0xAF,
            Opcode::Jp(Cond::Always, Op16::Immediate(_)) => 0xC3,
            Opcode::Unknown(opcode) => opcode,
            _ => panic!("Unimplemented"),
        }
    }

}
