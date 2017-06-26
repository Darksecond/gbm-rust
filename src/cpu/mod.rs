mod registers;
mod opcodes;

use self::opcodes::{Opcode, Op8, Op16, Addr};
use mmu::{MMU, Bus};
use cpu::registers::{Registers, Reg8, Reg16, Flags};

#[derive(Debug)]
pub enum Cond {
    NZ,
    NC,
    Z,
    C,
    Always,
}

impl Cond {
    fn check(&self, flags: Flags) -> bool {
        match *self {
            Cond::Always => true,
            Cond::Z => flags.contains(registers::Z),
            Cond::C => flags.contains(registers::C),
            Cond::NZ => !flags.contains(registers::Z),
            Cond::NC => !flags.contains(registers::C),
        }
    }
}

pub trait DecInc {
    fn dec(&mut self, cpu: &mut CPU);
    fn inc(&mut self, cpu: &mut CPU);
}

pub trait In16 {
    fn read(&self, &mut CPU) -> u16;
}

pub trait Out16 {
    fn write(&self, &mut CPU, u16);
}

pub trait In8 {
    fn read(&self, &mut CPU) -> u8;
}

pub trait Out8 {
    fn write(&self, &mut CPU, u8);
}

impl In16 for Reg16 {
    fn read(&self, cpu: &mut CPU) -> u16 {
        use cpu::registers::Reg16::*;
        match *self {
            HL => ((cpu.regs.h as u16) << 8) | (cpu.regs.l as u16),
            BC => ((cpu.regs.b as u16) << 8) | (cpu.regs.c as u16),
            DE => ((cpu.regs.d as u16) << 8) | (cpu.regs.e as u16),
            PC => cpu.regs.pc,
            SP => cpu.regs.sp,
        }
    }
}

impl Out16 for Reg16 {
    fn write(&self, cpu: &mut CPU, value: u16) {
        use cpu::registers::Reg16::*;
        match *self {
            HL => {
                cpu.regs.h = (value >> 8) as u8;
                cpu.regs.l = value as u8;
            },
            BC => {
                cpu.regs.b = (value >> 8) as u8;
                cpu.regs.c = value as u8;
            },
            DE => {
                cpu.regs.d = (value >> 8) as u8;
                cpu.regs.e = value as u8;
            },
            PC => cpu.regs.pc = value,
            SP => cpu.regs.sp = value,
        }
    }
}

impl DecInc for Reg16 {
    fn dec(&mut self, cpu: &mut CPU) {
        let value = self.read(cpu).wrapping_sub(1);
        self.write(cpu, value);
    }

    fn inc(&mut self, cpu: &mut CPU) {
        let value = self.read(cpu).wrapping_add(1);
        self.write(cpu, value);
    }
}

impl DecInc for Reg8 {
    fn dec(&mut self, cpu: &mut CPU) {
        let value = self.read(cpu).wrapping_sub(1);
        self.write(cpu, value);
    }

    fn inc(&mut self, cpu: &mut CPU) {
        let value = self.read(cpu).wrapping_add(1);
        self.write(cpu, value);
    }
}

impl In8 for Reg8 {
    fn read(&self, cpu: &mut CPU) -> u8 {
        use cpu::registers::Reg8::*;
        match *self {
            A => cpu.regs.a,
            B => cpu.regs.b,
            C => cpu.regs.c,
            D => cpu.regs.d,
            E => cpu.regs.e,
            H => cpu.regs.h,
            L => cpu.regs.l,
        }
    }
}

impl Out8 for Reg8 {
    fn write(&self, cpu: &mut CPU, value: u8) {
        use cpu::registers::Reg8::*;
        match *self {
            A => cpu.regs.a = value,
            B => cpu.regs.b = value,
            C => cpu.regs.c = value,
            D => cpu.regs.d = value,
            E => cpu.regs.e = value,
            H => cpu.regs.h = value,
            L => cpu.regs.l = value,
        }
    }
}

pub struct Immediate;
impl In16 for Immediate {
    fn read(&self, cpu: &mut CPU) -> u16 {
        cpu.next_u16()
    }
}

impl In8 for Immediate {
    fn read(&self, cpu: &mut CPU) -> u8 {
        cpu.next_u8()
    }
}

pub enum Memory {
    HL
}

impl Out8 for Memory {
    fn write(&self, cpu: &mut CPU, value: u8) {
        let addr = match self {
            HL => Reg16::HL.read(cpu)
        };
        cpu.mmu.write(addr, value)
    }
}

pub struct CPU<'a> {
    regs: Registers,
    mmu: &'a mut MMU<'a>
}

impl<'a> CPU<'a> {
    pub fn new(mmu: &'a mut MMU<'a>) -> CPU<'a> {
        CPU {
            regs: Registers::new(),
            mmu: mmu
        }
    }

    pub fn step(&mut self) {
        let (opcode, instruction) = Opcode::decode(self);
        println!("Regs   : {:?}", self.regs);
        println!("PC     : 0x{0:04x}", self.regs.pc-1);
        println!("Opcode : {:?} (0x{:x})", instruction, opcode);
        self.decode(instruction);
    }

    pub fn next_u8(&mut self) -> u8 {
        let addr = self.regs.pc;
        self.regs.pc += 1;
        self.mmu.read(addr)
    }

    pub fn next_u16(&mut self) -> u16 {
        let l = self.next_u8();
        let h = self.next_u8();
        ((h as u16) << 8) | (l as u16)
    }

    fn decode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Nop => (),
            Opcode::Jr(cond, addr) => self.jr(cond, addr),
            Opcode::Jp(cond, to) => self.jp(cond, to),
            Opcode::Xor(Op8::Register(reg)) => self.xor(reg),
            Opcode::Ld16(to, from) => self.load16(from, to),
            Opcode::Ld(to, from) => self.load8(from, to),
            Opcode::Dec(Op8::Register(reg)) => self.dec8(reg),
            Opcode::Inc(reg) => self.inc8(reg),
            Opcode::Adc(to, from) => self.adc(from, to),
            Opcode::Rra => self.rra(),
            _ => panic!("Unknown opcode ({:?})", opcode),
        }
    }
}

impl In16 for Op16 {
    fn read(&self, cpu: &mut CPU) -> u16 {
        match *self {
            Op16::Register(ref r) => r.read(cpu),
            Op16::Immediate(value) => value,
            _ => panic!("Not yet implemented (Op16+In16) ({:?})", self),
        }
    }
}

impl Out16 for Op16 {
    fn write(&self, cpu: &mut CPU, value: u16) {
        match *self {
            Op16::Register(ref r) => r.write(cpu, value),
            Op16::Immediate(_) => panic!("You cannot write to an immediate"),
            _ => panic!("Not yet implemented (Op16+Out16) ({:?})", self),
        }
    }
}

impl In8 for Op8 {
    fn read(&self, cpu: &mut CPU) -> u8 {
        match *self {
            Op8::Register(ref r) => r.read(cpu),
            Op8::Immediate(value) => value,
            _ => panic!("Not yet implemented (Op8+In8) ({:?})", self),
        }
    }
}

impl Out8 for Op8 {
    fn write(&self, cpu: &mut CPU, value: u8) {
        match *self {
            Op8::Register(ref r) => r.write(cpu, value),
            Op8::Immediate(_) => panic!("You cannot write to an immediate"),
            Op8::Memory(Addr::HLD) => {Memory::HL.write(cpu, value); Reg16::HL.dec(cpu); },
            Op8::Memory(Addr::HLI) => {Memory::HL.write(cpu, value); Reg16::HL.inc(cpu); },
            _ => panic!("Not yet implemented (Op8+Out8) ({:?})", self),
        }
    }
}

impl DecInc for Op8 {
    fn dec(&mut self, cpu: &mut CPU) {
        match *self {
            Op8::Register(ref mut r) => r.dec(cpu),
            _ => panic!("Not yet implemented (Op8+DecInc+dec) ({:?})", self),
        }
    }
    fn inc(&mut self, cpu: &mut CPU) {
        match *self {
            Op8::Register(ref mut r) => r.inc(cpu),
            _ => panic!("Not yet implemented (Op8+DecInc+inc) ({:?})", self),
        }
    }
}

impl<'a> CPU<'a> {
    fn xor<I: In8>(&mut self, in8: I) {
        let value = in8.read(self);
        self.regs.a = self.regs.a ^ value;
        self.regs.f = registers::Z.test(self.regs.a == 0); // (Z 0 0 0)
    }
    
    fn rra(&mut self) {
        let value = self.regs.a;
        let ci = if self.regs.f.contains(registers::C) {
            1
        } else {
            0
        };
        let co = value & 0x01;
        self.regs.a = (value >> 1) | (ci << 7);
        self.regs.f = registers::C.test(co != 0);
    }

    fn jr(&mut self, cond: Cond, addr: u8) {
        if cond.check(self.regs.f) {
            self.regs.pc += addr as u16;
        }
    }

    fn jp<I: In16>(&mut self, cond: Cond, addr: I) {
        if cond.check(self.regs.f) {
            self.regs.pc = addr.read(self);
        }
    }

    fn adc<I: In8, O: In8+Out8>(&mut self, in8: I, out8: O) {
        let value = in8.read(self);
        let original = out8.read(self);
        let c = if self.regs.f.contains(registers::C) {1} else {0};
        let result = original.wrapping_add(value).wrapping_add(c);
        self.regs.f = registers::Z.test(result == 0) |
            registers::C.test(original as u16 + value as u16 + c as u16 > 0xff) |
            registers::H.test((original & 0xf) + (value & 0xf) + c > 0xf);
        out8.write(self, result);
    }

    fn dec8<I: DecInc+In8>(&mut self, mut reg: I) {
        reg.dec(self);
        let value = reg.read(self);

        self.regs.f =
            registers::Z.test(value == 0) | // Z
            registers::N | // 1
            registers::H.test((value & 0x0F) == 0x0F) | // H
            (self.regs.f & registers::C); // -
    }

    fn inc8<I: DecInc+In8>(&mut self, mut in8: I) {
        in8.inc(self);
        let value = in8.read(self);
        self.regs.f =
            registers::Z.test(value == 0) | // Z
            registers::H.test((value & 0x0F) == 0x0F) | // H
            (self.regs.f & registers::C); // -
    }

    fn load16<I: In16, O: Out16>(&mut self, in16: I, out16: O) {
        let value = in16.read(self);
        out16.write(self, value);
    }

    fn load8<I: In8, O: Out8>(&mut self, in8: I, out8: O) {
        let value = in8.read(self);
        out8.write(self, value);
    }
}
