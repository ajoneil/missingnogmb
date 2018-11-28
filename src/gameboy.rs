use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::rom_info::RomInfo;
use crate::video::Video;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Gameboy {
    info: RomInfo,
    cpu: Cpu,
    mmu: Mmu,
    video: Rc<RefCell<Video>>,
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Gameboy {
        let info = RomInfo::new(rom.as_slice());
        let cartridge = Cartridge::new(rom, info.mbc_type);
        let video = Rc::new(RefCell::new(Video::new()));
        let mmu = Mmu::new(cartridge, video.clone());

        let gb = Gameboy {
            info: info,
            cpu: Cpu::new(),
            mmu: mmu,
            video: video,
        };

        println!("{}", gb.info.title);

        gb
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.mmu);
            println!("{:?}", self.cpu);
        }
    }
}
