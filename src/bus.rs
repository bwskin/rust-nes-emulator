use crate::rom::Rom;

pub trait Bus {
    fn mem_read(&self, addr: u16) -> u8 ;
    fn mem_write(&mut self, addr: u16, data: u8) -> ();

    fn mem_read_u16(&self, addr: u16) -> u16 {
        (self.mem_read(addr.wrapping_add(1)) as u16) << 8 | (self.mem_read(addr) as u16)
    }

    fn mem_read_u16_zero_page(&self, addr: u16) -> u16 {
        (self.mem_read(addr.wrapping_add(1) % 256) as u16) << 8 | (self.mem_read(addr % 256) as u16)
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.mem_write(addr, (data & 0xff) as u8);
        self.mem_write(addr + 1, (data >> 8) as u8);
    }
}

pub struct TestBus {
    memory: [u8; 65536]
}

impl Bus for TestBus {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl TestBus {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            memory: [0u8; 65536]
        })
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_to_specific_address(&mut self, addr: u16, program: Vec<u8>) {
        self.memory[(addr as usize)..(addr as usize + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, addr);
    }
}


pub struct NesBus {
    ram: [u8; 2048],
    rom: Rom
}

impl NesBus {
    pub fn new(rom: Rom) -> Box<Self> {
        Box::new(Self {
            ram: [0u8; 2048],
            rom
        })
    }

    pub fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl Bus for NesBus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[addr as usize],
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => todo!()
        }
        
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram[addr as usize] = data,
            0x8000..=0xFFFF => panic!("Attempt to write ROM space"),
            _ => todo!()
        }
    }
}
