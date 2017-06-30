#[macro_use]
extern crate bitflags;

mod cartridge;
mod memory;
mod mmu;
mod cpu;
mod gpu;

use cartridge::Cartridge;
use mmu::MMU;
use cpu::CPU;

//TODO Investigate minifb
fn main() {
    use std::env;
    let filename = env::args().nth(1).expect("Missing argument");
    println!("{}", filename);
    let cart = Cartridge::new(&filename).unwrap();
    println!("{}", cart.title());
    let mut mmu = MMU::new(cart);

    // CPU
    let mut cpu = CPU::new(&mut mmu);
    loop {
        cpu.step();
    }
}
