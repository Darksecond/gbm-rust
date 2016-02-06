mod cartridge;
mod mmu;

use cartridge::Cartridge;
use mmu::MMU;

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
}
