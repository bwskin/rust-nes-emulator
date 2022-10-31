#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    Indirect,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

pub struct AddressResult {
    pub address: u16,
    pub page_crossed: bool,
}

impl crate::cpu::CPU {
    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn stack_push(&mut self, value: u8) {
        self.mem_write((0x01 << 8) + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.overflowing_sub(1).0;
    }

    pub fn stack_push_u16(&mut self, value: u16) {
        self.mem_write((0x01 << 8) + self.stack_pointer as u16, (value >> 8) as u8);
        self.mem_write(
            (0x01 << 8) + self.stack_pointer.overflowing_sub(1).0 as u16,
            (value & 0xff) as u8,
        );
        self.stack_pointer = self.stack_pointer.overflowing_sub(2).0;
    }

    pub fn stack_pop(&mut self) -> u8 {
        let value = self.mem_read((0x01 << 8) + self.stack_pointer.overflowing_add(1).0 as u16);
        self.stack_pointer = self.stack_pointer.overflowing_add(1).0;
        value
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.mem_read((0x01 << 8) + self.stack_pointer.overflowing_add(1).0 as u16);
        let hi = self.mem_read((0x01 << 8) + self.stack_pointer.overflowing_add(2).0 as u16);
        self.stack_pointer = self.stack_pointer.overflowing_add(2).0;
        ((hi as u16) << 8) + lo as u16
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        (self.mem_read(addr.wrapping_add(1)) as u16) << 8 | (self.mem_read(addr) as u16)
    }

    pub fn mem_read_u16_zero_page(&self, addr: u16) -> u16 {
        (self.mem_read(addr.wrapping_add(1) % 256) as u16) << 8 | (self.mem_read(addr % 256) as u16)
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.mem_write(addr, (data & 0xff) as u8);
        self.mem_write(addr + 1, (data >> 8) as u8);
    }

    pub fn pop_next(&mut self) -> u16 {
        let value = self.program_counter;
        self.program_counter = self.program_counter.wrapping_add(1);
        value as u16
    }

    pub fn pop_read(&mut self) -> u8 {
        let addr = self.pop_next();
        self.mem_read(addr)
    }

    pub fn pop_read_u16(&mut self) -> u16 {
        self.pop_read() as u16 + ((self.pop_read() as u16) << 8)
    }

    pub fn get_operand_address(&mut self, mode: &AddressingMode) -> AddressResult {
        match mode {
            AddressingMode::Immediate => AddressResult {
                address: self.pop_next(),
                page_crossed: false,
            },
            AddressingMode::Indirect => AddressResult {
                address: {
                    let address = self.mem_read_u16(self.program_counter);

                    // This is to replicate bug that occurs in 6502 JMP indirect addressing
                    // https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
                    if address & 0xFF == 0xFF && self.jmp_compat {
                        ((self.mem_read(address & 0xFF00) as u16) << 8)
                            + self.mem_read(address) as u16
                    } else {
                        self.mem_read_u16(address)
                    }
                },
                page_crossed: false,
            },
            AddressingMode::Relative => AddressResult {
                address: {
                    let offset = self.pop_read();
                    if offset & 0b1000_0000 == 0 {
                        self.program_counter.wrapping_add(offset as u16)
                    } else {
                        self.program_counter
                            .wrapping_sub(((offset ^ 0xFFu8) + 1u8) as u16)
                    }
                },
                page_crossed: false,
            },
            AddressingMode::ZeroPage => AddressResult {
                address: self.pop_read() as u16,
                page_crossed: false,
            },
            AddressingMode::Absolute => AddressResult {
                address: self.pop_read_u16(),
                page_crossed: false,
            },

            AddressingMode::ZeroPageX => AddressResult {
                address: self.pop_read().wrapping_add(self.register_x) as u16,
                page_crossed: false
            },
            AddressingMode::ZeroPageY => AddressResult {
                address: self.pop_read().wrapping_add(self.register_y) as u16,
                page_crossed: false
            },
            AddressingMode::AbsoluteX => {
                let immediate_address_part = self.pop_read_u16();
                AddressResult {
                    address: immediate_address_part.wrapping_add(self.register_x as u16),
                    page_crossed: (immediate_address_part & 0xFF) + (self.register_x as u16) > 255
                }
            },
            AddressingMode::AbsoluteY => {
                let immediate_address_part = self.pop_read_u16();
                AddressResult {
                    address: immediate_address_part.wrapping_add(self.register_y as u16),
                    page_crossed: (immediate_address_part & 0xFF) + (self.register_y as u16) > 255
                }
            },
            AddressingMode::IndirectX => AddressResult {
                address: {
                    let immediate_address_part = self.pop_read();
                    self.mem_read_u16_zero_page(
                        immediate_address_part.wrapping_add(self.register_x) as u16
                    )
                },
                page_crossed: false
            },
            AddressingMode::IndirectY => {
                let immediate_address_part = self.pop_read();
                let indirect_address_no_index = self.mem_read_u16_zero_page(immediate_address_part as u16);

                AddressResult {
                    address: indirect_address_no_index.wrapping_add(self.register_y as u16),
                    page_crossed: (indirect_address_no_index & 0xFF) + (self.register_y as u16) > 255
                }
            }
            AddressingMode::NoneAddressing => panic!("Mode {:?} is not supported", mode),
        }
    }
}
