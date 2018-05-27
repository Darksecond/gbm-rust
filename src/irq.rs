
#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    VBlank  = 0b00000001,
    LcdStat = 0b00000010,
    Timer   = 0b00000100,
    Serial  = 0b00001000,
    Joypad  = 0b00010000,
}

impl Interrupt {
    fn from_u8(value: u8) -> Option<Interrupt> {
        match value {
            0b00000001 => Some(Interrupt::VBlank),
            0b00000010 => Some(Interrupt::LcdStat),
            0b00000100 => Some(Interrupt::Timer),
            0b00001000 => Some(Interrupt::Serial),
            0b00010000 => Some(Interrupt::Joypad),
            _ => None,
        }
    }

    pub fn addr(&self) -> u16 {
        match self {
            &Interrupt::VBlank => 0x40,
            &Interrupt::LcdStat => 0x48,
            &Interrupt::Timer => 0x50,
            &Interrupt::Serial => 0x58,
            &Interrupt::Joypad => 0x60,
        }
    }
}

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

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.request |= Interrupts::from_bits_truncate(interrupt as u8);
    }

    pub fn has_interrupt(&self) -> bool {
        self.enable & self.request != Interrupts::empty()
    }

    pub fn ack_interrupt(&mut self) -> Option<Interrupt> {
        let ints = self.get_enable() & self.get_request();
        let int = ints & ints.wrapping_neg();
        self.request -= Interrupts::from_bits_truncate(int);
        Interrupt::from_u8(int)
    }
}