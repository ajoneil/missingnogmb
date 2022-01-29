use crate::cartridge::Cartridge;
use crate::video::Video;

pub struct Mmu {
    cartridge: Cartridge,
    wram: [u8; 0x2000],
    hram: [u8; 0x7f],
    interrupt_flag: u8,
    interrupt_enable: u8,
}

pub struct Mapper<'a> {
    mmu: &'a mut Mmu,
    video: &'a mut Video,
}

impl Mapper<'_> {
    pub fn new<'a>(mmu: &'a mut Mmu, video: &'a mut Video) -> Mapper<'a> {
        Mapper { mmu, video }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mmu.read(address, self.video)
    }

    pub fn read_pc(&self, pc: &mut u16) -> u8 {
        let val = self.read(*pc);
        *pc += 1;
        val
    }

    pub fn read_word(&self, address: u16) -> u16 {
        self.mmu.read_word(address, self.video)
    }

    pub fn read_word_pc(&self, pc: &mut u16) -> u16 {
        let val = self.read_word(*pc);
        *pc += 2;
        val
    }

    pub fn write(&mut self, address: u16, val: u8) {
        self.mmu.write(address, val, self.video)
    }

    pub fn write_word(&mut self, address: u16, val: u16) {
        self.mmu.write_word(address, val, self.video)
    }
}

impl Mmu {
    pub fn new(cartridge: Cartridge) -> Mmu {
        Mmu {
            cartridge: cartridge,
            wram: [0; 0x2000],
            hram: [0; 0x7f],
            interrupt_flag: 0x00,
            interrupt_enable: 0x00,
        }
    }

    pub fn read(&self, address: u16, video: &Video) -> u8 {
        match address {
            0x0000..=0x7fff => self.cartridge.read(address),
            0xc000..=0xdfff => self.wram[address as usize - 0xc000],
            0xe000..=0xfdff => self.wram[address as usize - 0xe000],
            0xff0f => self.interrupt_flag,
            0xff01..=0xff02 => 0x00, // link cable NYI
            0xff40..=0xff4a => video.read(address),
            0xff80..=0xfffe => self.hram[address as usize - 0xff80],
            0xffff => self.interrupt_enable,
            _ => panic!("Unimplemented read from {:x}", address),
        }
    }

    pub fn read_word(&self, address: u16, video: &Video) -> u16 {
        self.read(address, video) as u16 + (self.read(address + 1, video) as u16 * 0x100)
    }

    pub fn write(&mut self, address: u16, val: u8, video: &mut Video) {
        match address {
            0xc000..=0xdfff => self.wram[address as usize - 0xc000] = val,
            0xe000..=0xfdff => self.wram[address as usize - 0xe000] = val,
            0xff01..=0xff02 => {} // link cable, NYI
            0xff0f => self.interrupt_flag = val,
            0xff10..=0xff26 => {} // sound, nyi
            0xff40..=0xff4a => video.write(address, val),
            0xff80..=0xfffe => self.hram[address as usize - 0xff80] = val,
            0xffff => self.interrupt_enable = val,
            _ => panic!("Unimplemented write to {:x}", address),
        }
    }

    pub fn write_word(&mut self, address: u16, val: u16, video: &mut Video) {
        self.write(address, (val & 0xff) as u8, video);
        self.write(address + 1, (val >> 8) as u8, video);
    }
}
