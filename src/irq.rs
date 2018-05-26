
//TODO Rename to Interrupt
bitflags!(
    pub struct Interrupts: u8 {
        const INT_VBLANK  = 0b00000001;
        const INT_LCDSTAT = 0b00000010;
        const INT_TIMER   = 0b00000100;
        const INT_SERIAL  = 0b00001000;
        const INT_JOYPAD  = 0b00010000;
    }
);

pub struct Irq {
    enable: Interrupts,
    request: Interrupts,
}

impl Irq {
    pub fn new() -> Irq {
        Irq {
            enable: Interrupts::empty(),
            request: Interrupts::empty(),
        }
    }

    pub fn set_enable(&mut self, bits: u8) {
        self.enable = Interrupts::from_bits_truncate(bits);
    }

    pub fn get_enable(&self) -> u8 {
        self.enable.bits()
    }

    pub fn set_request(&mut self, bits: u8) {
        self.request = Interrupts::from_bits_truncate(bits);
    }

    pub fn get_request(&self) -> u8 {
        self.request.bits()
    }

    pub fn request_interrupt(&mut self, interrupt: Interrupts) {
        self.request |= interrupt;
    }
}