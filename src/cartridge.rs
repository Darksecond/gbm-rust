use mmu::Bus;

pub struct Cartridge {
    rom: Vec<u8>
}

impl Cartridge {
    pub fn new(filename : &String) -> Result<Cartridge,String> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = try!(File::open(filename).map_err(|e| { format!("{}", e)}) );
        let mut buffer = vec!();
        try!(file.read_to_end(&mut buffer).map_err(|e| { format!("{}", e)}) );

        Ok(Cartridge {
            rom: buffer
        })
    }

    pub fn title(&self) -> String {
        use std::str;
        let slice = &self.rom[0x134 .. 0x143];
        str::from_utf8(slice).map_err(|_| { "".to_string() }).unwrap().trim_right_matches('\0').to_string()
    }
}

impl Bus for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&mut self, _: u16, _: u8) {
        //panic!("Write not supported");
    }

    fn cycle(&mut self) {}
}
