
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
    BC, DE,
    Immediate(u16),
}

#[derive(Debug)]
pub enum Op16 {
    Register(Reg16),
    Immediate(u16),
    Memory(Addr),
}

#[derive(Debug)]
pub enum Opcode {
    Unknown(u8),
    Nop,
    Dec(Op8),
    Dec16(Op16),
    Xor(Op8),
    Jp(Cond, Op16),
    Jr(Cond, u8),
    Ld(Op8, Op8),
    Ld16(Op16, Op16),
    Or(Op8),
    Inc16(Op16),
    Inc(Op8),
    Rra,
    Rla,
    Rlca,
    Rrca,
    Add16(Op16, Op16),
    Stop(u8),
}

impl Opcode {
    //TODO Implement cylces
    pub fn decode(cpu: &mut CPU) -> (u8, Opcode) {
        let opcode = cpu.next_u8();
        let instruction = match opcode {
            0x00 => Opcode::Nop,
            0x01 => Opcode::Ld16(Op16::Register(Reg16::BC), Op16::Immediate(cpu.next_u16())),
            0x02 => Opcode::Ld(Op8::Memory(Addr::BC), Op8::Register(Reg8::A)),
            0x03 => Opcode::Inc16(Op16::Register(Reg16::BC)),
            0x04 => Opcode::Inc(Op8::Register(Reg8::B)),
            0x05 => Opcode::Dec(Op8::Register(Reg8::B)),
            0x06 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Immediate(cpu.next_u8())),
            0x07 => Opcode::Rlca,
            0x08 => Opcode::Ld16(Op16::Memory(Addr::Immediate(cpu.next_u16())), Op16::Register(Reg16::SP)),
            0x09 => Opcode::Add16(Op16::Register(Reg16::HL), Op16::Register(Reg16::BC)),
            0x0A => Opcode::Ld(Op8::Register(Reg8::A), Op8::Memory(Addr::BC)),
            0x0B => Opcode::Dec16(Op16::Register(Reg16::BC)),
            0x0C => Opcode::Inc(Op8::Register(Reg8::C)),
            0x0D => Opcode::Dec(Op8::Register(Reg8::C)),
            0x0E => Opcode::Ld(Op8::Register(Reg8::C), Op8::Immediate(cpu.next_u8())),
            0x0F => Opcode::Rrca,

            0x10 => Opcode::Stop(cpu.next_u8()),
            0x11 => Opcode::Ld16(Op16::Register(Reg16::DE), Op16::Immediate(cpu.next_u16())),
            0x12 => Opcode::Ld(Op8::Memory(Addr::DE), Op8::Register(Reg8::A)),
            0x13 => Opcode::Inc16(Op16::Register(Reg16::DE)),
            0x14 => Opcode::Inc(Op8::Register(Reg8::D)),
            0x15 => Opcode::Dec(Op8::Register(Reg8::D)),
            0x16 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Immediate(cpu.next_u8())),
            0x17 => Opcode::Rla,
            0x18 => Opcode::Jr(Cond::Always, cpu.next_u8()),
            0x19 => Opcode::Add16(Op16::Register(Reg16::HL), Op16::Register(Reg16::DE)),
            0x1A => Opcode::Ld(Op8::Register(Reg8::A), Op8::Memory(Addr::DE)),
            0x1B => Opcode::Dec16(Op16::Register(Reg16::DE)),
            0x1C => Opcode::Inc(Op8::Register(Reg8::E)),
            0x1D => Opcode::Dec(Op8::Register(Reg8::E)),
            0x1E => Opcode::Ld(Op8::Register(Reg8::E), Op8::Immediate(cpu.next_u8())),
            0x1F => Opcode::Rra,

            0x20 => Opcode::Jr(Cond::NZ, cpu.next_u8()),
            0x21 => Opcode::Ld16(Op16::Register(Reg16::HL), Op16::Immediate(cpu.next_u16())),
            0x25 => Opcode::Dec(Op8::Register(Reg8::H)),

            0x32 => Opcode::Ld(Op8::Memory(Addr::HLD), Op8::Register(Reg8::A)),

            0xAF => Opcode::Xor(Op8::Register(Reg8::A)),

            0xB0 => Opcode::Or(Op8::Register(Reg8::B)),

            0xC3 => Opcode::Jp(Cond::Always, Op16::Immediate(cpu.next_u16())),
            _ => Opcode::Unknown(opcode),
        };
        (opcode, instruction)
    }
}
