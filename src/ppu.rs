use crate::cpu::instructions::flags::FlagRegister;
use crate::flag;
use crate::rom::Mirroring;
use paste::paste;

pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: (0, 0),
            hi_ptr: true,
        }
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn increment(&mut self, inc: u8) {
        let (value, overflow) = self.value.1.overflowing_add(inc);
        self.value.1 = value;
        if overflow {
            self.value.0.wrapping_add(1);
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}

pub struct ControlRegister {
    status: u8,
}

impl FlagRegister for ControlRegister {
    fn get_flag(&self, flag_mask: u8) -> u8 {
        if (self.status & flag_mask) != 0 {
            1
        } else {
            0
        }
    }
    fn set_flag(&mut self, flag_mask: u8, value: u8) {
        if value == 1 {
            self.status = self.status | flag_mask;
        } else {
            self.status = self.status & !flag_mask;
        }
    }
}

impl ControlRegister {
    flag! { nametable1            , 0b00000001 }
    flag! { nametable2            , 0b00000010 }
    flag! { vram_add_increment    , 0b00000100 }
    flag! { sprite_pattern_addr   , 0b00001000 }
    flag! { backround_pattern_addr, 0b00010000 }
    flag! { sprite_size           , 0b00100000 }
    flag! { master_slave_select   , 0b01000000 }
    flag! { generate_nmi          , 0b10000000 }

    pub fn vram_addr_increment(&self) -> u8 {
        if self.get_vram_add_increment_flag() == 0 {
            1
        } else {
            32
        }
    }

    pub fn update(&mut self, data: u8) {
        self.status = data;
    }
}

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub addr: AddrRegister,
    pub ctrl: ControlRegister,
    internal_data_buf: u8
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            addr: AddrRegister::new(),
            ctrl: ControlRegister { status: 0 },
            internal_data_buf: 0
        }
    }

    fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111;
        let vram_index = mirrored_vram - 0x2000;
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index
        }
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            },
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            },
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {}",
                addr
            ),
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr)
        }
    }
}
