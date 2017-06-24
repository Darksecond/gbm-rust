#[macro_use]
extern crate bitflags;

mod cartridge;
mod memory;
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

    // CPU
    let mut cpu = CPU::new(&mut mmu);
    while true {
        cpu.step();
    }
}
