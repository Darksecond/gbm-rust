use mmu::Bus;

#[derive(Debug)]
pub enum Mode {
    VBlank,
    HBlank,
    ReadOam,
    ReadVram,
}

bitflags!(
    pub struct Control: u8 {
        const BG_ON        = 0b00000001;
        const OBJ_ON       = 0b00000010;
        const OBJ_SIZE     = 0b00000100;
        const BG_MAP_BASE  = 0b00001000;
        const BG_TILE_BASE = 0b00010000;
        const WND_ON       = 0b00100000;
        const WND_MAP_BASE = 0b01000000;
        const LCD_ON       = 0b10000000;
    }
);

pub struct Gpu {
    scroll_y: u8,
    scroll_x: u8,
    current_line: u8,
    control: Control,
    mode: Mode,
    cycles: usize,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            scroll_y: 0,
            scroll_x: 0,
            current_line: 0,
            control: Control::empty(),
            mode: Mode::ReadVram,
            cycles: 0,
        }
    }
}

impl Bus for Gpu {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.control.bits(),
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.current_line,
            _ => panic!("Not yet implemented read 0x{:04x}", addr)
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF40 => self.control = Control::from_bits_truncate(value),
            0xFF41 => println!("GPU 0xFF41 = 0x{:02x} STAT WRITE NOT YET IMPLEMENTED", value),
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => self.current_line = 0,
            _ => panic!("Not yet implemented write 0x{:04x} = 0x{:02x}", addr, value)
        }
    }

    fn cycle(&mut self) {
        self.cycles += 1;

        match self.mode {
            Mode::VBlank => {
                if(self.cycles >= 114) {
                    self.cycles -= 114;
                    self.current_line += 1;
                    if(self.current_line > 153)  {
                        self.mode = Mode::ReadOam;
                        self.current_line = 0;
                    }
                }
            },
            Mode::HBlank => {
                if(self.cycles >= 51) {
                    self.cycles -= 51;
                    self.current_line += 1;
                    if(self.current_line == 144) {
                        self.mode =  Mode::VBlank;
                    } else {
                        self.mode = Mode::ReadOam;
                    }
                }
            },
            Mode::ReadOam => {
                if(self.cycles >= 20) {
                    self.cycles -= 20;
                    self.mode = Mode::ReadVram;
                }
            },
            Mode::ReadVram => {
                if(self.cycles >= 43) {
                    self.cycles -= 43;
                    self.mode = Mode::HBlank;
                }
            },
        }
    }
}
