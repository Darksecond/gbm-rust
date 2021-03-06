mod registers;
mod opcodes;

use self::opcodes::{Opcode, Op8, Op16, Addr};
use mmu::{MMU, Bus, Master};
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
            AF => ((cpu.regs.a as u16) << 8) | (cpu.regs.f.bits() as u16),
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
            AF => {
                cpu.regs.a = (value >> 8) as u8;
                cpu.regs.f = Flags::from_bits_truncate(value as u8);
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

#[derive(Copy, Clone)]
pub enum Memory {
    HL,
    HLI,
    HLD,
    DE,
}

impl Out8 for Memory {
    fn write(&self, cpu: &mut CPU, value: u8) {
        let addr = match self {
            &Memory::HL | &Memory::HLI | &Memory::HLD => Reg16::HL.read(cpu),
            &Memory::DE => Reg16::DE.read(cpu),
        };
        cpu.write_u8(addr, value);
        match self {
            &Memory::HLI => Reg16::HL.inc(cpu),
            &Memory::HLD => Reg16::HL.dec(cpu),
            _ => (),
        };
    }
}

impl In8 for Memory {
    fn read(&self, cpu: &mut CPU) -> u8 {
        let addr = match self {
            &Memory::HL | &Memory::HLI | &Memory::HLD => Reg16::HL.read(cpu),
            &Memory::DE => Reg16::DE.read(cpu),
        };
        let value = cpu.read_u8(addr);
        match self {
            &Memory::HLI => Reg16::HL.inc(cpu),
            &Memory::HLD => Reg16::HL.dec(cpu),
            _ => (),
        };
        value
    }
}

enum Ime {
    Disabled,
    Enabled,
    Enabling,
}

//TODO Use Bus+Master instead of MMU
//TODO Don't save a reference
pub struct CPU<'a> {
    regs: Registers,
    ime: Ime,
    mmu: &'a mut MMU
}

impl<'a> CPU<'a> {
    pub fn new(mmu: &'a mut MMU) -> CPU<'a> {
        CPU {
            regs: Registers::new(),
            ime: Ime::Disabled,
            mmu: mmu
        }
    }

    pub fn step(&mut self) {
        let pc = self.regs.pc;
        //println!("Regs   : {}", self.regs);
        let opcode = self.read_u8(pc);
        let interrupt = match self.ime {
            Ime::Disabled | Ime::Enabling => false,
            Ime::Enabled => self.mmu.has_interrupt()
        };
        match self.ime {
            Ime::Enabling => self.ime = Ime::Enabled,
            _ => (),
        }
        
        if interrupt {
            //println!("INTERRUPT");
            self.dispatch_interrupt();
        } else {
            self.regs.pc += 1;
            let (opcode, instruction) = Opcode::decode(self, opcode);
            //if pc >= 0x0293 && pc <= 0x029e {
            //    println!("Regs   : {}", self.regs);
            //    println!("[0x{:04x}] 0x{:02x} ({:?})", pc, opcode, instruction);
            //}
            self.decode(instruction);
        }
    }

    fn dispatch_interrupt(&mut self) {
        //self.halt = false;
        self.ime = Ime::Disabled;
        self.mmu.cycle();
        self.mmu.cycle();
        let pc = self.regs.pc;
        self.push_u16(pc);
        if let Some(interrupt) = self.mmu.ack_interrupt() {
            self.regs.pc = interrupt.addr();
        } else {
            self.regs.pc = 0x0000;
        }
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.mmu.cycle();
        self.mmu.write(addr, value);
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let l = self.read_u8(addr);
        let h = self.read_u8(addr+1);
        ((h as u16) << 8) | (l as u16)
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        self.write_u8(addr, value as u8);
        self.write_u8(addr+1, (value >> 8) as u8);
    }

    pub fn read_u8(&mut self, addr: u16) -> u8 {
        self.mmu.cycle();
        self.mmu.read(addr)
    }

    pub fn next_u8(&mut self) -> u8 {
        let addr = self.regs.pc;
        self.regs.pc += 1;
        self.read_u8(addr)
    }

    pub fn next_u16(&mut self) -> u16 {
        let l = self.next_u8();
        let h = self.next_u8();
        ((h as u16) << 8) | (l as u16)
    }

    fn push_u8(&mut self, value: u8) {
        Reg16::SP.dec(self);
        let sp = Reg16::SP.read(self);
        self.write_u8(sp, value)
    }

    fn push_u16(&mut self, value: u16) {
        self.push_u8((value >> 8) as u8);
        self.push_u8(value as u8);
    }

    fn pop_u8(&mut self) -> u8 {
        let sp = Reg16::SP.read(self);
        let value = self.read_u8(sp);
        Reg16::SP.inc(self);
        value
    }

    fn pop_u16(&mut self) -> u16 {
        let l = self.pop_u8();
        let h = self.pop_u8();
        ((h as u16) << 8) | (l as u16)
    }

    fn decode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Nop => (),
            Opcode::Jr(cond, addr) => self.jr(cond, addr),
            Opcode::Jp(cond, to) => self.jp(cond, to),
            Opcode::Xor(Op8::Register(reg)) => self.xor(reg),
            Opcode::Ld16(to, from) => self.load16(from, to), //Ld16(SP,HL) needs an internal cycle ?!?
            Opcode::Ld(to, from) => self.load8(from, to),
            Opcode::Dec(op) => self.dec8(op),
            Opcode::Dec16(Op16::Register(reg)) => self.dec16(reg),
            Opcode::Inc16(Op16::Register(reg)) => self.inc16(reg),
            Opcode::Inc(reg) => self.inc8(reg),
            Opcode::Adc(to, from) => self.adc(from, to),
            Opcode::Rra => self.rra(),
            Opcode::Di => self.ime = Ime::Disabled,
            Opcode::Cp(op) => self.cp(op),
            Opcode::Call(Cond::Always, addr) => self.call(addr),
            Opcode::Ret(Cond::Always) => self.ret(),
            Opcode::Ret(cond) => self.ret_cond(cond),
            Opcode::Or(Op8::Register(reg)) => self.or(reg),
            Opcode::Ei => self.ime = Ime::Enabling,
            Opcode::Cpl => self.cpl(),
            Opcode::And(from) => self.and(from),
            Opcode::Swap(op) => self.swap(op),
            Opcode::Rst(addr) => self.rst(addr),
            Opcode::Add(to, from) => self.add8(from, to),
            Opcode::Add16(to, from) => self.add16(from, to),
            Opcode::Unknown(opcode) => panic!("Unknown opcode 0x{:04X}", opcode),
            Opcode::Pop(Op16::Register(reg)) => self.pop(reg),
            Opcode::Push(Op16::Register(reg)) => self.push(reg),
            Opcode::Res(bit, op) => self.res(bit, op),
            Opcode::Scf => self.scf(),
            _ => panic!("Unhandled opcode ({:?})", opcode),
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
            Op8::Memory(Addr::ZeroPage(addr)) => cpu.read_u8(0xFF00|(addr as u16)),
            Op8::Memory(Addr::HLI) => Memory::HLI.read(cpu),
            Op8::Memory(Addr::HL) => Memory::HL.read(cpu),
            Op8::Memory(Addr::DE) => Memory::DE.read(cpu),
            Op8::Memory(Addr::Immediate(addr)) => cpu.read_u8(addr),
            _ => panic!("Not yet implemented (Op8+In8) ({:?})", self),
        }
    }
}

impl Out8 for Op8 {
    fn write(&self, cpu: &mut CPU, value: u8) {
        match *self {
            Op8::Register(ref r) => r.write(cpu, value),
            Op8::Immediate(_) => panic!("You cannot write to an immediate"),
            Op8::Memory(Addr::HL) => Memory::HL.write(cpu, value),
            Op8::Memory(Addr::HLD) => Memory::HLD.write(cpu, value),
            Op8::Memory(Addr::HLI) => Memory::HLI.write(cpu, value),
            Op8::Memory(Addr::ZeroPage(addr)) => {cpu.write_u8(0xFF00|(addr as u16), value);},
            Op8::Memory(Addr::ZeroPageC) => { let c = Reg8::C.read(cpu) as u16; cpu.write_u8(0xFF00|c, value)},
            Op8::Memory(Addr::Immediate(addr)) => {cpu.write_u8(addr, value);},
            Op8::Memory(Addr::DE) => {let addr = Reg16::DE.read(cpu); cpu.write_u8(addr, value); },
            _ => panic!("Not yet implemented (Op8+Out8) ({:?})", self),
        }
    }
}

impl DecInc for Op8 {
    fn dec(&mut self, cpu: &mut CPU) {
        match *self {
            Op8::Register(ref mut r) => r.dec(cpu),
            Op8::Memory(Addr::HL) => {
                let value = Memory::HL.read(cpu).wrapping_sub(1);
                Memory::HL.write(cpu, value);
            },
            _ => panic!("Not yet implemented (Op8+DecInc+dec) ({:?})", self),
        }
    }
    fn inc(&mut self, cpu: &mut CPU) {
        match *self {
            Op8::Register(ref mut r) => r.inc(cpu),
            Op8::Memory(Addr::HL) => {
                let value = Memory::HL.read(cpu).wrapping_add(1);
                Memory::HL.write(cpu, value);
            },
            _ => panic!("Not yet implemented (Op8+DecInc+inc) ({:?})", self),
        }
    }
}

impl<'a> CPU<'a> {
    fn scf(&mut self) {
        self.regs.f = (self.regs.f & registers::Z) | registers::C;
    }

    fn res<IO: In8+Out8>(&mut self, bit: u8, io8: IO) {
        let value = io8.read(self);
        let mask = !(1 << bit);
        let value = value & mask;
        io8.write(self, value);
    }

    fn push(&mut self, reg: Reg16) {
        let value = reg.read(self);
        self.mmu.cycle();
        self.push_u16(value);
    }

    fn add16<I: In16, O: In16+Out16>(&mut self, in16: I, out16: O) {
        let rhs = in16.read(self);
        let lhs = out16.read(self);
        let (value, carry) = lhs.overflowing_add(rhs);
        let half_carry = (((lhs >> 4) & 0xf)+((rhs >> 4) & 0xf)) & 0x10 == 0x10;
        self.regs.f = (self.regs.f & registers::Z) |
            registers::H.test(half_carry) |
            registers::C.test(carry);
        out16.write(self, value);
    }

    fn pop(&mut self, reg: Reg16) {
        let value = self.pop_u16();
        reg.write(self, value);
    }

    fn add8<I: In8, O: In8+Out8>(&mut self, in8: I, out8: O) {
        let rhs = in8.read(self);
        let lhs = out8.read(self);
        let (value, carry) = lhs.overflowing_add(rhs);
        let half_carry = ((lhs & 0xf)+(rhs & 0xf)) & 0x10 == 0x10;
        self.regs.f = registers::Z.test(value == 0) |
            registers::H.test(half_carry) |
            registers::C.test(carry);
        out8.write(self, value);
    }

    fn rst(&mut self, addr: u8) {
        let pc = Reg16::PC.read(self);
        self.mmu.cycle();
        self.push_u16(pc);
        Reg16::PC.write(self, addr as u16);
    }

    fn swap<IO: In8+Out8>(&mut self, op: IO) {
        let value = op.read(self);
        let value = (value >> 4) | (value << 4);
        self.regs.f = registers::Z.test(value == 0); // (Z 0 0 0)
        op.write(self, value);
    }

    fn dec16<R: DecInc+In16>(&mut self, mut reg: R) {
        reg.dec(self);
        self.mmu.cycle();
    }

    fn inc16<R: DecInc+In16>(&mut self, mut reg: R) {
        reg.inc(self);
        self.mmu.cycle();
    }

    fn ret_cond(&mut self, cond: Cond) {
        self.mmu.cycle();
        if cond.check(self.regs.f) {
            let pc = self.pop_u16();
            Reg16::PC.write(self, pc);
            self.mmu.cycle();
        }
    }

    fn ret(&mut self) {
        let pc = self.pop_u16();
        Reg16::PC.write(self, pc);
        self.mmu.cycle();
    }

    fn call<I: In16>(&mut self, addr: I) {
        let value = addr.read(self);
        let pc = Reg16::PC.read(self);
        self.mmu.cycle();
        self.push_u16(pc);
        Reg16::PC.write(self, value);
    }

    fn cp<I: In8>(&mut self, op: I) {
        let value = op.read(self);
        let result = self.regs.a.wrapping_sub(value);
        self.regs.f = registers::Z.test(result == 0) |
            registers::N |
            registers::H.test((self.regs.a & 0xf) < (value & 0xf)) |
            registers::C.test((self.regs.a as u16) < (value as u16));
    }

    fn and<I: In8>(&mut self, in8: I) {
        let value = in8.read(self);
        self.regs.a = self.regs.a & value;
        self.regs.f = registers::Z.test(self.regs.a == 0) | 
            registers::H; // (Z 0 1 0)
    }

    fn or<I: In8>(&mut self, in8: I) {
        let value = in8.read(self);
        self.regs.a = self.regs.a | value;
        self.regs.f = registers::Z.test(self.regs.a == 0); // (Z 0 0 0)
    }

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
            self.regs.pc = self.regs.pc.wrapping_add((addr as i8) as u16);
        }
    }

    fn jp<I: In16>(&mut self, cond: Cond, addr: I) {
        if cond.check(self.regs.f) {
            self.regs.pc = addr.read(self);
        }
    }

    fn cpl(&mut self) {
        self.regs.a = !self.regs.a;
        self.regs.f = (self.regs.f & registers::Z) | // -
            registers::N | // 1
            registers::H | // 1
            (self.regs.f & registers::C); // -
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
