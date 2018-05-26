use mmu::Bus;
use memory::Ram;
use mmu::InterruptCycle;
use irq::Irq;

pub enum Color {
    White = 0,
    Light = 1,
    Dark  = 2,
    Black = 3,
}

impl Color {
    pub fn from_u8(value: u8) -> Color {
        use gpu::Color::*;
        match value {
            1 => Light,
            2 => Dark,
            3 => Black,
            _ => White,
        }
    }
}

//wwxxyyzz
//^- 11 Black
//  ^- 10 Dark
//    ^- 01 Light
//      ^- 00 White
struct Palette {
    black: Color,
    dark: Color,
    light: Color,
    white: Color,
}

impl Palette {
    fn from_u8(value: u8) -> Palette {
        Palette {
            black: Color::from_u8((value >> 6) & 0x3),
            dark:  Color::from_u8((value >> 4) & 0x3),
            light: Color::from_u8((value >> 2) & 0x3),
            white: Color::from_u8((value >> 0) & 0x3),
        }
    }
}

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
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    vram: Ram,
    oam: Ram,
    window_y: u8,
    window_x: u8,
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
            bg_palette:   Palette::from_u8(0b11111100),
            obj0_palette: Palette::from_u8(0b11111111),
            obj1_palette: Palette::from_u8(0b11111111),
            vram: Ram::new(8192),
            oam: Ram::new(160),
            window_y: 0,
            window_x: 0,
        }
    }
}

impl Bus for Gpu {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000 ... 0x9FFF => self.vram.read(addr & 0x1FFF),
            0xFE00 ... 0xFE9F => self.oam.read(addr & 0xFF),
            0xFF40 => self.control.bits(),
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.current_line,
            _ => panic!("Not yet implemented read 0x{:04x}", addr)
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000 ... 0x9FFF => self.vram.write(addr & 0x1FFF, value),
            0xFE00 ... 0xFE9F => self.oam.write(addr & 0xFF, value),
            0xFF40 => self.control = Control::from_bits_truncate(value),
            0xFF41 => println!("GPU 0xFF41 = 0x{:02x} STAT WRITE NOT YET IMPLEMENTED", value),
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => self.current_line = 0,
            0xFF47 => self.bg_palette = Palette::from_u8(value),
            0xFF48 => self.obj0_palette = Palette::from_u8(value),
            0xFF49 => self.obj1_palette = Palette::from_u8(value),
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            _ => panic!("Not yet implemented write 0x{:04x} = 0x{:02x}", addr, value)
        }
    }
}

impl InterruptCycle for Gpu {
    fn cycle(&mut self, irq: &mut Irq) {
        if !self.control.contains(LCD_ON) {
            return;
        }

        self.cycles += 1;

        match self.mode {
            Mode::VBlank => {
                if self.cycles >= 114 {
                    self.cycles -= 114;
                    self.current_line += 1;
                    if self.current_line > 153 {
                        self.mode = Mode::ReadOam;
                        self.current_line = 0;
                    }
                }
            },
            Mode::HBlank => {
                if self.cycles >= 51 {
                    self.cycles -= 51;
                    self.current_line += 1;
                    if self.current_line == 144 {
                        self.mode =  Mode::VBlank;
                        irq.request_interrupt(::irq::INT_VBLANK);
                    } else {
                        self.mode = Mode::ReadOam;
                    }
                }
            },
            Mode::ReadOam => {
                if self.cycles >= 20 {
                    self.cycles -= 20;
                    self.mode = Mode::ReadVram;
                }
            },
            Mode::ReadVram => {
                if self.cycles >= 43 {
                    self.cycles -= 43;
                    self.mode = Mode::HBlank;
                }
            },
        }
    }
}
