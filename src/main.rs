#[macro_use]
extern crate bitflags;

mod cartridge;
mod mmu;
mod cpu;

use cartridge::Cartridge;
use mmu::MMU;
use cpu::CPU;

fn main() {
    use std::env;
    let filename = env::args().nth(1).expect("Missing argument");
    println!("{}", filename);
    let cart = Cartridge::new(&filename).unwrap();
    let mut mmu = MMU::new(&cart);
    println!("{}", cart.title());

    // MMU test
    mmu.write(0xC000, 123);
    println!("{}", mmu.read(0xC000));

    // CPU
    let mut cpu = CPU::new(&mut mmu);
    cpu.step(); // NOP
    cpu.step(); // JP
    cpu.step(); // JP
    cpu.step(); // XOR A
    cpu.step(); // LD HL, nn
    cpu.step(); // LD C, n
    cpu.step(); // LD B, n
    cpu.step(); // LDD (HL), A
    cpu.step();
}
