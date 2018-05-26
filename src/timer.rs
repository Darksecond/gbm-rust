use mmu::Bus;

//TODO DO proper implementation

pub struct Timer {
    enabled: bool,
    input_clock: u8, //TODO use enum
    modulo: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            enabled: false,
            input_clock: 0,
            modulo: 0,
        }
    }

    fn set_control(&mut self, value: u8) {
        self.enabled = (value & 0x04) != 0;
        self.input_clock = value & 0x03;
    }

    fn set_modulo(&mut self, value: u8) {
        self.modulo = value;
    }
}

impl Bus for Timer {
    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF06 => self.set_modulo(value),
            0xFF07 => self.set_control(value),
           _ => panic!("Unsupported write 0x{:04x} = 0x{:02x}", addr, value)
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            _ => panic!("Unsupported read 0x{:04x}", addr)
        }
    }
}