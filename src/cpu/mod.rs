mod registers;

use mmu::{MMU, Bus};
use cpu::registers::{Registers, Reg8, Reg16, Flags};

trait DecInc {
    fn dec(&mut self, cpu: &mut CPU);
    fn inc(&mut self, cpu: &mut CPU);
}

trait In16 {
    fn read(&self, &mut CPU) -> u16;
}

trait Out16 {
    fn write(&self, &mut CPU, u16);
}

trait In8 {
    fn read(&self, &mut CPU) -> u8;
}

trait Out8 {
    fn write(&self, &mut CPU, u8);
}

impl In16 for Reg16 {
    fn read(&self, cpu: &mut CPU) -> u16 {
        use cpu::registers::Reg16::*;
        match *self {
            HL => ((cpu.regs.h as u16) << 8) | (cpu.regs.l as u16),
            PC => cpu.regs.pc
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
            PC => cpu.regs.pc = value
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
            C => cpu.regs.c
        }
    }
}

impl Out8 for Reg8 {
    fn write(&self, cpu: &mut CPU, value: u8) {
        use cpu::registers::Reg8::*;
        match *self {
            A => cpu.regs.a = value,
            B => cpu.regs.b = value,
            C => cpu.regs.c = value
        }
    }
}

struct Immediate;
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

enum Memory {
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
        let opcode = self.next_u8();
        println!("{:?}", self.regs);
        println!("0x{0:04x}: 0x{1:02x}", self.regs.pc-1, opcode);
        self.decode(opcode);
    }

    fn next_u8(&mut self) -> u8 {
        let addr = self.regs.pc;
        self.regs.pc += 1;
        self.mmu.read(addr)
    }

    fn next_u16(&mut self) -> u16 {
        let l = self.next_u8();
        let h = self.next_u8();
        ((h as u16) << 8) | (l as u16)
    }

    fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00 => (), // NOP
            0x05 => self.dec8(Reg8::B), // DEC B
            0x06 => self.load8(Immediate, Reg8::B), // LD B, n
            0x0E => self.load8(Immediate, Reg8::C), // LD C, n
            0x21 => self.load16(Immediate, Reg16::HL), // LD HL, nn
            0x32 => {self.load8(Reg8::A, Memory::HL); Reg16::HL.dec(self); }, // LD (HL--), A
            0xAF => self.xor(Reg8::A), // XOR A
            0xC3 => self.regs.pc = self.next_u16(), // JP nn
            _ => panic!("Unknown opcode: 0x{0:02x}", opcode)
        }
    }
}

impl<'a> CPU<'a> {
    fn xor<I: In8>(&mut self, in8: I) {
        let value = in8.read(self);
        self.regs.a = self.regs.a ^ value;
        self.regs.f = registers::Z.test(self.regs.a == 0); // (Z 0 0 0)
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

    fn load16<I: In16, O: Out16>(&mut self, in16: I, out16: O) {
        let value = in16.read(self);
        out16.write(self, value);
    }

    fn load8<I: In8, O: Out8>(&mut self, in8: I, out8: O) {
        let value = in8.read(self);
        out8.write(self, value);
    }
}
