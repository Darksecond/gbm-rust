#[macro_use]
extern crate bitflags;

mod rom;
mod cartridge;
mod memory;
mod mmu;
mod cpu;
mod gpu;
mod timer;
mod irq;

use cartridge::Cartridge;
use mmu::MMU;
use cpu::CPU;

//TODO Investigate minifb
//TODO Use actual bios
//     Use boot-values for stuff in that case!
//TODO Overhaul cycle architecture or at least test it
fn main() {
    use std::env;
    let filename = env::args().nth(1).expect("Missing argument");
    println!("{}", filename);
    let cart = Cartridge::new(&filename).unwrap();
    println!("{}", cart.title());
    println!("MBC: {:?}", cart.memory_bank_controller());
    let mut mmu = MMU::new(cart);

    // CPU
    let mut cpu = CPU::new(&mut mmu);
    loop {
        cpu.step();
    }
}
