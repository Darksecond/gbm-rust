use mmu::Bus;
use rom::Rom;

//TODO Support banking

#[derive(Debug, Copy, Clone)]
pub enum MemoryBankController
{
    RomOnly,
    Mbc1,
}

impl MemoryBankController {
    fn from_u8(value: u8) -> MemoryBankController {
        match value {
            0 => MemoryBankController::RomOnly,
            1 => MemoryBankController::Mbc1,
            _ => panic!("Unknown Memory Bank Controller {}", value),
        }
    }
}

pub struct Cartridge {
    rom: Rom,
    mbc: MemoryBankController
}

impl Cartridge {
    pub fn new(filename : &String) -> Result<Cartridge,String> {
        let rom = Rom::new(filename)?;
        Ok(Cartridge {
            mbc: MemoryBankController::from_u8(rom.as_slice()[0x147]),
            rom,
        })
    }

    pub fn memory_bank_controller(&self) -> MemoryBankController {
        self.mbc
    }

    pub fn title(&self) -> String {
        use std::str;
        let slice = &self.rom.as_slice()[0x134 .. 0x143];
        str::from_utf8(slice).map_err(|_| { "".to_string() }).unwrap().trim_right_matches('\0').to_string()
    }
}

impl Bus for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        self.rom.read(addr)
    }

    fn write(&mut self, _addr: u16, _value: u8) {
        match self.memory_bank_controller() {
            MemoryBankController::RomOnly => (),
            MemoryBankController::Mbc1 => panic!("Not yet implemented"),
        }
    }

    fn cycle(&mut self) {}
}
