pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen
}

const NES_TAG: &[u8] = b"NES";

const PRG_BANK_SIZE: usize = 16384;
const VROM_BANK_SIZE: usize = 8192;

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl TryFrom <Vec<u8>> for Rom {
    type Error = String;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        (&bytes).try_into()
    }
}

impl TryFrom <&Vec<u8>> for Rom {
    type Error = String;
    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        if &bytes[0..4] != NES_TAG {
            return Err("File is not in iNES format".to_string())
        };

        let mapper = (bytes[7] & 0b1111_0000) | (bytes[6] >> 4);

        let ines_version = (bytes[7] >> 2) & 0b11;

        if ines_version != 0 {
            return Err("iNES 2.0 is not supported".to_string());
        };

        let screen_mirroring = match (
            bytes[6] & 0b1000 != 0, // Four screen layout
            bytes[6] & 0b1000 != 0  // Vertical mirroring
        ) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal
        };

        let prg_rom_size = bytes[4] as usize * PRG_BANK_SIZE;
        let chr_rom_size = bytes[5] as usize * VROM_BANK_SIZE;

        let skip_trainer = bytes[6] & 0b100 != 0;

        let prg_rom_start = if skip_trainer { 528 } else { 16 };
        let chr_rom_start = prg_rom_start+prg_rom_size;

        Ok(Self {
            prg_rom: bytes[prg_rom_start..prg_rom_start+prg_rom_size].into(),
            chr_rom: bytes[chr_rom_start..chr_rom_start+chr_rom_size].into(),
            mapper,
            screen_mirroring
        })
    }
}
