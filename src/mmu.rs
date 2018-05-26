use cartridge::Cartridge;
use memory::Ram;
use gpu::Gpu;
use timer::Timer;

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
    fn new() -> Irq {
        Irq {
            enable: Interrupts::empty(),
            request: Interrupts::empty(),
        }
    }

    fn set_enable(&mut self, bits: u8) {
        self.enable = Interrupts::from_bits_truncate(bits);
    }


    fn get_enable(&self) -> u8 {
        self.enable.bits()
    }

    fn set_request(&mut self, bits: u8) {
        self.request = Interrupts::from_bits_truncate(bits);
    }

    fn get_request(&self) -> u8 {
        self.request.bits()
    }
}


pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
    fn cycle(&mut self);
}

pub struct MMU {
    cart: Cartridge,
    wram: Ram,
    zram: Ram,
    irq: Irq,
    gpu: Gpu,
    timer: Timer,
}

impl MMU {
    pub fn new(cart: Cartridge) -> MMU {
        MMU {
            cart: cart,
            wram: Ram::new(8192),
            zram: Ram::new(128),
            irq: Irq::new(),
            gpu: Gpu::new(),
            timer: Timer::new(),
        }
    }
}

impl Bus for MMU {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x3FFF => self.cart.read(addr),
            0x4000 ... 0x7FFF => self.cart.read(addr),
            0x8000 ... 0x9FFF => self.gpu.read(addr),
            0xC000 ... 0xDFFF => self.wram.read(addr & 0x1FFF),
            0xE000 ... 0xFDFF => self.wram.read(addr & 0x1FFF),
            0xFE00 ... 0xFE9F => self.gpu.read(addr),
            0xFF00 => 0x00, //TODO Joypad
            0xFF04 ... 0xFF07 => self.timer.read(addr),
            0xFF0F => self.irq.get_enable(),
            0xFF40 ... 0xFF55 => self.gpu.read(addr),
            0xFF80 ... 0xFFFE => self.zram.read(addr & 0x7F),
            0xFFFF => self.irq.get_request(),
            _ => panic!("Unsupported read 0x{:04x}", addr)
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000 ... 0x3FFF => self.cart.write(addr, value),
            0x4000 ... 0x7FFF => self.cart.write(addr, value),
            0x8000 ... 0x9FFF => self.gpu.write(addr, value),
            0xC000 ... 0xDFFF => self.wram.write(addr & 0x1FFF, value),
            0xE000 ... 0xFDFF => self.wram.write(addr & 0x1FFF, value),
            0xFE00 ... 0xFE9F => self.gpu.write(addr, value),
            0xFEA0 ... 0xFEFF => (), //TODO unusable
            0xFF00 => (), //TODO Joypad
            0xFF01 ... 0xFF02 => (), //TODO serial
            0xFF04 ... 0xFF07 => self.timer.write(addr, value),
            0xFF0F => self.irq.set_enable(value),
            0xFF10 ... 0xFF26 => (), //TODO Sound
            0xFF40 ... 0xFF55 => self.gpu.write(addr, value),
            0xFF7F => (), //TODO unknown
            0xFF80 ... 0xFFFE => self.zram.write(addr & 0x7F, value),
            0xFFFF => self.irq.set_request(value),
            _ => panic!("Unsupported write 0x{:04x} = 0x{:02x}", addr, value)
        }
    }

    fn cycle(&mut self) {
        self.gpu.cycle();
    }
}
