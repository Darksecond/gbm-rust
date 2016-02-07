mod registers;

use mmu::MMU;
use cpu::registers::{Registers, Reg8, Reg16, Flags};

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

impl In8 for Reg8 {
    fn read(&self, cpu: &mut CPU) -> u8 {
        use cpu::registers::Reg8::*;
        match *self {
            A => cpu.regs.a
        }
    }
}

impl Out8 for Reg8 {
    fn write(&self, cpu: &mut CPU, value: u8) {
        use cpu::registers::Reg8::*;
        match *self {
            A => cpu.regs.a = value
        }
    }
}

struct Immediate16;
impl In16 for Immediate16 {
    fn read(&self, cpu: &mut CPU) -> u16 {
        cpu.next_u16()
    }
}

pub struct CPU<'a> {
    regs: Registers,
    mmu: &'a MMU<'a>
}

impl<'a> CPU<'a> {
    pub fn new(mmu: &'a MMU) -> CPU<'a> {
        CPU {
            regs: Registers::new(),
            mmu: mmu
        }
    }

    pub fn step(&mut self) {
        let opcode = self.next_u8();
        println!("0x{0:x}: 0x{1:x}", self.regs.pc-1, opcode);
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
            0x21 => self.load16(Immediate16, Reg16::HL), // LD HL, nn
            0xAF => self.xor(Reg8::A), // XOR A
            0xC3 => self.regs.pc = self.next_u16(), // JP nn
            _ => panic!("Unknown opcode: 0x{0:x}", opcode)
        }
    }
}

impl<'a> CPU<'a> {
    fn xor<I: In8>(&mut self, in8: I) {
        let value = in8.read(self);
        self.regs.a = self.regs.a ^ value;
        self.regs.f = registers::Z.test(self.regs.a == 0);
    }

    fn load16<I: In16, O:Out16>(&mut self, in16: I, out16: O) {
        let value = in16.read(self);
        out16.write(self, value);
    }
}
