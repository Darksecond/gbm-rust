
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
    ZeroPage(u8),
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
    Add(Op8, Op8),
    Add16(Op16, Op16),
    Adc(Op8, Op8),
    Stop(u8),
    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
    Sub(Op8),
    Sbc(Op8, Op8),
    And(Op8),
    Cp(Op8),
    Ret(Cond),
    Pop(Op16),
    Push(Op16),
    Call(Cond, Op16),
    Rst(u8),
    Di,
    Ei,
}

impl Opcode {
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
            0x22 => Opcode::Ld(Op8::Memory(Addr::HLI), Op8::Register(Reg8::A)),
            0x23 => Opcode::Inc16(Op16::Register(Reg16::HL)),
            0x24 => Opcode::Inc(Op8::Register(Reg8::H)),
            0x25 => Opcode::Dec(Op8::Register(Reg8::H)),
            0x26 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Immediate(cpu.next_u8())),
            0x27 => Opcode::Daa,
            0x28 => Opcode::Jr(Cond::Z, cpu.next_u8()),
            0x29 => Opcode::Add16(Op16::Register(Reg16::HL), Op16::Register(Reg16::HL)),
            0x2A => Opcode::Ld(Op8::Register(Reg8::A), Op8::Memory(Addr::HLI)),
            0x2B => Opcode::Dec16(Op16::Register(Reg16::HL)),
            0x2C => Opcode::Inc(Op8::Register(Reg8::L)),
            0x2D => Opcode::Dec(Op8::Register(Reg8::L)),
            0x2E => Opcode::Ld(Op8::Register(Reg8::L), Op8::Immediate(cpu.next_u8())),
            0x2F => Opcode::Cpl,

            0x30 => Opcode::Jr(Cond::NC, cpu.next_u8()),
            0x31 => Opcode::Ld16(Op16::Register(Reg16::SP), Op16::Immediate(cpu.next_u16())),
            0x32 => Opcode::Ld(Op8::Memory(Addr::HLD), Op8::Register(Reg8::A)),
            0x33 => Opcode::Inc16(Op16::Register(Reg16::SP)),
            0x34 => Opcode::Inc(Op8::Memory(Addr::HL)),
            0x35 => Opcode::Dec(Op8::Memory(Addr::HL)),
            0x36 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Immediate(cpu.next_u8())),
            0x37 => Opcode::Scf,
            0x38 => Opcode::Jr(Cond::C, cpu.next_u8()),
            0x39 => Opcode::Add16(Op16::Register(Reg16::HL), Op16::Register(Reg16::SP)),
            0x3A => Opcode::Ld(Op8::Register(Reg8::A), Op8::Memory(Addr::HLD)),
            0x3B => Opcode::Dec16(Op16::Register(Reg16::SP)),
            0x3C => Opcode::Inc(Op8::Register(Reg8::A)),
            0x3D => Opcode::Dec(Op8::Register(Reg8::A)),
            0x3E => Opcode::Ld(Op8::Register(Reg8::A), Op8::Immediate(cpu.next_u8())),
            0x3F => Opcode::Ccf,
            
            0x40 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::B)),
            0x41 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::C)),
            0x42 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::D)),
            0x43 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::E)),
            0x44 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::H)),
            0x45 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::L)),
            0x46 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Memory(Addr::HL)),
            0x47 => Opcode::Ld(Op8::Register(Reg8::B), Op8::Register(Reg8::A)),
            0x48 => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::B)),
            0x49 => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::C)),
            0x4A => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::D)),
            0x4B => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::E)),
            0x4C => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::H)),
            0x4D => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::L)),
            0x4E => Opcode::Ld(Op8::Register(Reg8::C), Op8::Memory(Addr::HL)),
            0x4F => Opcode::Ld(Op8::Register(Reg8::C), Op8::Register(Reg8::A)),

            0x50 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::B)),
            0x51 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::C)),
            0x52 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::D)),
            0x53 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::E)),
            0x54 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::H)),
            0x55 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::L)),
            0x56 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Memory(Addr::HL)),
            0x57 => Opcode::Ld(Op8::Register(Reg8::D), Op8::Register(Reg8::A)),
            0x58 => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::B)),
            0x59 => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::C)),
            0x5A => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::D)),
            0x5B => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::E)),
            0x5C => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::H)),
            0x5D => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::L)),
            0x5E => Opcode::Ld(Op8::Register(Reg8::E), Op8::Memory(Addr::HL)),
            0x5F => Opcode::Ld(Op8::Register(Reg8::E), Op8::Register(Reg8::A)),

            0x60 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::B)),
            0x61 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::C)),
            0x62 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::D)),
            0x63 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::E)),
            0x64 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::H)),
            0x65 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::L)),
            0x66 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Memory(Addr::HL)),
            0x67 => Opcode::Ld(Op8::Register(Reg8::H), Op8::Register(Reg8::A)),
            0x68 => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::B)),
            0x69 => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::C)),
            0x6A => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::D)),
            0x6B => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::E)),
            0x6C => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::H)),
            0x6D => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::L)),
            0x6E => Opcode::Ld(Op8::Register(Reg8::L), Op8::Memory(Addr::HL)),
            0x6F => Opcode::Ld(Op8::Register(Reg8::L), Op8::Register(Reg8::A)),

            0x70 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::B)),
            0x71 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::C)),
            0x72 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::D)),
            0x73 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::E)),
            0x74 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::H)),
            0x75 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::L)),
            0x76 => Opcode::Halt,
            0x77 => Opcode::Ld(Op8::Memory(Addr::HL), Op8::Register(Reg8::A)),
            0x78 => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::B)),
            0x79 => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::C)),
            0x7A => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::D)),
            0x7B => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::E)),
            0x7C => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::H)),
            0x7D => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::L)),
            0x7E => Opcode::Ld(Op8::Register(Reg8::A), Op8::Memory(Addr::HL)),
            0x7F => Opcode::Ld(Op8::Register(Reg8::A), Op8::Register(Reg8::A)),

            0x80 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::B)),
            0x81 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::C)),
            0x82 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::D)),
            0x83 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::E)),
            0x84 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::H)),
            0x85 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::L)),
            0x86 => Opcode::Add(Op8::Register(Reg8::A), Op8::Memory(Addr::HL)),
            0x87 => Opcode::Add(Op8::Register(Reg8::A), Op8::Register(Reg8::A)),
            0x88 => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::B)),
            0x89 => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::C)),
            0x8A => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::D)),
            0x8B => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::E)),
            0x8C => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::H)),
            0x8D => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::L)),
            0x8E => Opcode::Adc(Op8::Register(Reg8::A), Op8::Memory(Addr::HL)),
            0x8F => Opcode::Adc(Op8::Register(Reg8::A), Op8::Register(Reg8::A)),

            0x90 => Opcode::Sub(Op8::Register(Reg8::B)),
            0x91 => Opcode::Sub(Op8::Register(Reg8::C)),
            0x92 => Opcode::Sub(Op8::Register(Reg8::D)),
            0x93 => Opcode::Sub(Op8::Register(Reg8::E)),
            0x94 => Opcode::Sub(Op8::Register(Reg8::H)),
            0x95 => Opcode::Sub(Op8::Register(Reg8::L)),
            0x96 => Opcode::Sub(Op8::Memory(Addr::HL)),
            0x97 => Opcode::Sub(Op8::Register(Reg8::A)),
            0x98 => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::B)),
            0x99 => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::C)),
            0x9A => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::D)),
            0x9B => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::E)),
            0x9C => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::H)),
            0x9D => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::L)),
            0x9E => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Memory(Addr::HL)),
            0x9F => Opcode::Sbc(Op8::Register(Reg8::A), Op8::Register(Reg8::A)),

            0xA0 => Opcode::And(Op8::Register(Reg8::B)),
            0xA1 => Opcode::And(Op8::Register(Reg8::C)),
            0xA2 => Opcode::And(Op8::Register(Reg8::D)),
            0xA3 => Opcode::And(Op8::Register(Reg8::E)),
            0xA4 => Opcode::And(Op8::Register(Reg8::H)),
            0xA5 => Opcode::And(Op8::Register(Reg8::L)),
            0xA6 => Opcode::And(Op8::Memory(Addr::HL)),
            0xA7 => Opcode::And(Op8::Register(Reg8::A)),

            0xA8 => Opcode::Xor(Op8::Register(Reg8::B)),
            0xA9 => Opcode::Xor(Op8::Register(Reg8::C)),
            0xAA => Opcode::Xor(Op8::Register(Reg8::D)),
            0xAB => Opcode::Xor(Op8::Register(Reg8::E)),
            0xAC => Opcode::Xor(Op8::Register(Reg8::H)),
            0xAD => Opcode::Xor(Op8::Register(Reg8::L)),
            0xAE => Opcode::Xor(Op8::Memory(Addr::HL)),
            0xAF => Opcode::Xor(Op8::Register(Reg8::A)),

            0xB0 => Opcode::Or(Op8::Register(Reg8::B)),
            0xB1 => Opcode::Or(Op8::Register(Reg8::C)),
            0xB2 => Opcode::Or(Op8::Register(Reg8::D)),
            0xB3 => Opcode::Or(Op8::Register(Reg8::E)),
            0xB4 => Opcode::Or(Op8::Register(Reg8::H)),
            0xB5 => Opcode::Or(Op8::Register(Reg8::L)),
            0xB6 => Opcode::Or(Op8::Memory(Addr::HL)),
            0xB7 => Opcode::Or(Op8::Register(Reg8::A)),
            0xB8 => Opcode::Cp(Op8::Register(Reg8::B)),
            0xB9 => Opcode::Cp(Op8::Register(Reg8::C)),
            0xBA => Opcode::Cp(Op8::Register(Reg8::D)),
            0xBB => Opcode::Cp(Op8::Register(Reg8::E)),
            0xBC => Opcode::Cp(Op8::Register(Reg8::H)),
            0xBD => Opcode::Cp(Op8::Register(Reg8::L)),
            0xBE => Opcode::Cp(Op8::Memory(Addr::HL)),
            0xBF => Opcode::Cp(Op8::Register(Reg8::A)),

            0xC0 => Opcode::Ret(Cond::NZ),
            0xC1 => Opcode::Pop(Op16::Register(Reg16::BC)),
            0xC2 => Opcode::Jp(Cond::NZ, Op16::Immediate(cpu.next_u16())),
            0xC3 => Opcode::Jp(Cond::Always, Op16::Immediate(cpu.next_u16())),
            0xC4 => Opcode::Call(Cond::NZ, Op16::Immediate(cpu.next_u16())),
            0xC5 => Opcode::Push(Op16::Register(Reg16::BC)),
            0xC6 => Opcode::Add(Op8::Register(Reg8::A), Op8::Immediate(cpu.next_u8())),
            0xC7 => Opcode::Rst(0x00),
            0xC8 => Opcode::Ret(Cond::Z),
            0xC9 => Opcode::Ret(Cond::Always),
            0xCA => Opcode::Jp(Cond::Z, Op16::Immediate(cpu.next_u16())),
                0xCB => panic!("Prefix CB Not Yet implemented!"), //TODO
            0xCC => Opcode::Call(Cond::Z, Op16::Immediate(cpu.next_u16())),
            0xCD => Opcode::Call(Cond::Always, Op16::Immediate(cpu.next_u16())),
            0xCE => Opcode::Adc(Op8::Register(Reg8::A), Op8::Immediate(cpu.next_u8())),
            0xCF => Opcode::Rst(0x08),

            0xE0 => Opcode::Ld(Op8::Memory(Addr::ZeroPage(cpu.next_u8())), Op8::Register(Reg8::A)),

            0xF0 => Opcode::Ld(Op8::Register(Reg8::A), Op8::Memory(Addr::ZeroPage(cpu.next_u8()))),
            0xF2 => Opcode::Pop(Op16::Register(Reg16::AF)),
            0xF3 => Opcode::Di,
            0xF8 => Opcode::Ei,
            0xFE => Opcode::Cp(Op8::Immediate(cpu.next_u8())),

            _ => Opcode::Unknown(opcode),
        };
        (opcode, instruction)
    }
}
