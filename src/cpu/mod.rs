mod registers;

use mmu::MMU;

pub struct CPU<'a> {
    regs: registers::Registers,
    mmu: &'a MMU<'a>
}

impl<'a> CPU<'a> {
    pub fn new(mmu: &'a MMU) -> CPU<'a> {
        CPU {
            regs: registers::Registers{pc: 0x100, a: 0x01},
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
            0xC3 => self.regs.pc = self.next_u16(), // JP
            _ => panic!("Unknown opcode: 0x{0:x}", opcode)
        }
    }
}
