use cartridge::Cartridge;
use memory::Ram;
use gpu::Gpu;
use timer::Timer;
use irq::Irq;

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}
pub trait Cycle { // Rename to BusMaster or something
    fn cycle(&mut self);
    //fn has_interrupt(&mut self) -> bool;
    //fn ack_interrupt(&mut self) -> Option<::irq::Interrupts>;
}
pub trait InterruptCycle { //TODO Rename to Device or Slave or BusSlave or something
    fn cycle(&mut self, irq: &mut Irq);
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
            0xFF00 => 0xFF, //TODO Joypad
            0xFF04 ... 0xFF07 => self.timer.read(addr),
            0xFF0F => self.irq.get_request(),
            0xFF40 ... 0xFF55 => self.gpu.read(addr),
            0xFF80 ... 0xFFFE => self.zram.read(addr & 0x7F),
            0xFFFF => self.irq.get_enable(),
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
            0xFF0F => self.irq.set_request(value),
            0xFF10 ... 0xFF26 => (), //TODO Sound
            0xFF40 ... 0xFF55 => self.gpu.write(addr, value),
            0xFF7F => (), //TODO unknown
            0xFF80 ... 0xFFFE => self.zram.write(addr & 0x7F, value),
            0xFFFF => self.irq.set_enable(value),
            _ => panic!("Unsupported write 0x{:04x} = 0x{:02x}", addr, value)
        }
    }
}

impl Cycle for MMU {
    fn cycle(&mut self) {
        self.gpu.cycle(&mut self.irq);
    }
}