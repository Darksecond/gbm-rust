use cartridge::Cartridge;
use memory::Ram;

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub struct MMU<'a> {
    cart: &'a Cartridge,
    wram: Ram,
}

impl<'a> MMU<'a> {
    pub fn new(cart: &Cartridge) -> MMU {
        MMU {
            cart: &cart,
            wram: Ram::new(8192),
        }
    }
}

impl<'a> Bus for MMU<'a> {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x3FFF => self.cart.read(addr),
            0x4000 ... 0x7FFF => self.cart.read(addr),
            0xC000 ... 0xDFFF => self.wram.read(addr & 0x1FFF),
            0xE000 ... 0xFDFF => self.wram.read(addr & 0x1FFF),
            _ => panic!("Unsupported read")
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xC000 ... 0xDFFF => self.wram.write(addr & 0x1FFF, value),
            0xE000 ... 0xFDFF => self.wram.write(addr & 0x1FFF, value),
            _ => panic!("Unsupported write")
        }
    }
}
