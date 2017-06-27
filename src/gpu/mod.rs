use mmu::Bus;

pub struct Gpu {
    scroll_x: u8,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            scroll_x: 0,
        }
    }
}

impl Bus for Gpu {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF43 => self.scroll_x,
            _ => panic!("Not yet implemented {:?}", addr)
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF43 => self.scroll_x = value,
            _ => panic!("Not yet implemented {:?}", addr)
        }
    }

    fn cycle(&mut self) {
    }
}
