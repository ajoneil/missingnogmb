use crate::cartridge::Cartridge;
use crate::video::Video;

pub struct Mmu {
    cartridge: Cartridge,
    wram: [u8; 0x2000],
    hram: [u8; 0x7f],
    interrupt_flag: u8,
    interrupt_enable: u8,
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
        self.write(address, (val % 0x100) as u8, video);
        self.write(address + 1, (val / 0x100) as u8, video);
    }
}
