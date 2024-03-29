pub struct RomInfo {
    pub title: String,
    pub mbc_type: MbcType,
    pub checksum: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum MbcType {
    NoMBC,
    MBC1,
    MBC2,
    MMM01,
    MBC3,
    MBC4,
    MBC5,
    Unknown,
}

impl RomInfo {
    pub fn new(rom: &[u8]) -> RomInfo {
        let mut title = String::new();
        for character in rom[0x134..0x144].iter() {
            if *character == 0u8 {
                break;
            }

            title.push(*character as char)
        }

        let mbc_type = match rom[0x147] {
            0x00 | 0x08 | 0x09 => MbcType::NoMBC,
            0x01..=0x03 => MbcType::MBC1,
            0x05 | 0x06 => MbcType::MBC2,
            0x0b..=0x0d => MbcType::MMM01,
            0x0f..=0x13 => MbcType::MBC3,
            0x15..=0x17 => MbcType::MBC4,
            0x19..=0x1e => MbcType::MBC5,
            _ => MbcType::Unknown,
        };

        RomInfo {
            title: title,
            mbc_type: mbc_type,
            checksum: rom[0x14d],
        }
    }
}
