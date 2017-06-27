use mmu::Bus;

pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    pub fn new(count: usize) -> Ram {
        Ram {
            ram: vec!(0; count)
        }
    }
}

impl Bus for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
    fn write(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    fn cycle(&mut self) {}
}
