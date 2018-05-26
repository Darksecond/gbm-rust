use mmu::Bus;

pub struct Rom {
    rom: Vec<u8>
}

impl Rom {
    pub fn new(filename : &String) -> Result<Rom,String> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = try!(File::open(filename).map_err(|e| { format!("{}", e)}) );
        let mut buffer = vec!();
        try!(file.read_to_end(&mut buffer).map_err(|e| { format!("{}", e)}) );

        Ok(Rom {
            rom: buffer
        })
    }

    pub fn as_slice(&self) -> &[u8] {
        self.rom.as_slice()
    }
}

impl Bus for Rom {
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        panic!("Write not supported: 0x{:x} = 0x{:x}", addr, value);
    }
}
