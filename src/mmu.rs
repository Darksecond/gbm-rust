use cartridge::Cartridge;

pub struct MMU<'a> {
    cart: &'a Cartridge,
    wram: Vec<u8>
}

impl<'a> MMU<'a> {
    pub fn new(cart: &Cartridge) -> MMU {
        MMU {
            cart: &cart,
            wram: vec!(0; 8192)
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x3FFF => self.cart.read(addr),
            0x4000 ... 0x7FFF => self.cart.read(addr),
            0xC000 ... 0xDFFF => self.wram[(addr & 0x1FFF) as usize],
            0xE000 ... 0xFDFF => self.wram[(addr & 0x1FFF) as usize],
            _ => panic!("Unsupported read")
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xC000 ... 0xDFFF => self.wram[(addr & 0x1FFF) as usize] = value,
            0xE000 ... 0xFDFF => self.wram[(addr & 0x1FFF) as usize] = value,
            _ => panic!("Unsupported write")
        }
    }
}
