// OPCODES! macro defined here
mod opcodes;
use opcodes::OPCODES;

// Implemented here:
// - zero flag handling
// - negative flag handling
mod flags;
use super::memory::{AddressResult, AddressingMode};

OPCODES! {
    // code, instruction name, cycles, addresing mode
    (0x69, "ADC", 2, AddressingMode::Immediate,      true)
    (0x65, "ADC", 3, AddressingMode::ZeroPage,       true)
    (0x75, "ADC", 4, AddressingMode::ZeroPageX,      true)
    (0x6D, "ADC", 4, AddressingMode::Absolute,       true)
    (0x7D, "ADC", 4, AddressingMode::AbsoluteX,      true)
    (0x79, "ADC", 4, AddressingMode::AbsoluteY,      true)
    (0x61, "ADC", 6, AddressingMode::IndirectX,      true)
    (0x71, "ADC", 5, AddressingMode::IndirectY,      true)

    (0x29, "AND", 2, AddressingMode::Immediate,      true)
    (0x25, "AND", 3, AddressingMode::ZeroPage,       true)
    (0x35, "AND", 4, AddressingMode::ZeroPageX,      true)
    (0x2D, "AND", 4, AddressingMode::Absolute,       true)
    (0x3D, "AND", 4, AddressingMode::AbsoluteX,      true)
    (0x39, "AND", 4, AddressingMode::AbsoluteY,      true)
    (0x21, "AND", 6, AddressingMode::IndirectX,      true)
    (0x31, "AND", 5, AddressingMode::IndirectY,      true)

    (0x0A, "ASL", 2, AddressingMode::NoneAddressing, true)
    (0x06, "ASL", 5, AddressingMode::ZeroPage,       true)
    (0x16, "ASL", 6, AddressingMode::ZeroPageX,      true)
    (0x0E, "ASL", 6, AddressingMode::Absolute,       true)
    (0x1E, "ASL", 7, AddressingMode::AbsoluteX,      true)

    (0x90, "BCC", 2, AddressingMode::Relative,       false)

    (0xB0, "BCS", 2, AddressingMode::Relative,       false)

    (0xF0, "BEQ", 2, AddressingMode::Relative,       false)

    (0x24, "BIT", 3, AddressingMode::ZeroPage,       true)
    (0x2C, "BIT", 4, AddressingMode::Absolute,       true)

    (0x30, "BMI", 2, AddressingMode::Relative,       false)

    (0xD0, "BNE", 2, AddressingMode::Relative,       false)

    (0x10, "BPL", 2, AddressingMode::Relative,       false)

    (0x00, "BRK", 7, AddressingMode::NoneAddressing, true)

    (0x50, "BVC", 2, AddressingMode::Relative,       false)

    (0x70, "BVS", 2, AddressingMode::Relative,       false)

    (0x18, "CLC", 2, AddressingMode::NoneAddressing, true)

    (0xD8, "CLD", 2, AddressingMode::NoneAddressing, true)

    (0x58, "CLI", 2, AddressingMode::NoneAddressing, true)

    (0xB8, "CLV", 2, AddressingMode::NoneAddressing, true)

    (0xC9, "CMP", 2, AddressingMode::Immediate,      true)
    (0xC5, "CMP", 3, AddressingMode::ZeroPage,       true)
    (0xD5, "CMP", 4, AddressingMode::ZeroPageX,      true)
    (0xCD, "CMP", 4, AddressingMode::Absolute,       true)
    (0xDD, "CMP", 4, AddressingMode::AbsoluteX,      true)
    (0xD9, "CMP", 4, AddressingMode::AbsoluteY,      true)
    (0xC1, "CMP", 6, AddressingMode::IndirectX,      true)
    (0xD1, "CMP", 5, AddressingMode::IndirectY,      true)

    (0xE0, "CPX", 2, AddressingMode::Immediate,      true)
    (0xE4, "CPX", 3, AddressingMode::ZeroPage,       true)
    (0xEC, "CPX", 4, AddressingMode::Absolute,       true)

    (0xC0, "CPY", 2, AddressingMode::Immediate,      true)
    (0xC4, "CPY", 3, AddressingMode::ZeroPage,       true)
    (0xCC, "CPY", 4, AddressingMode::Absolute,       true)

    (0xC6, "DEC", 5, AddressingMode::ZeroPage,       true)
    (0xD6, "DEC", 6, AddressingMode::ZeroPageX,      true)
    (0xCE, "DEC", 6, AddressingMode::Absolute,       true)
    (0xDE, "DEC", 7, AddressingMode::AbsoluteX,      true)

    (0xCA, "DEX", 2, AddressingMode::NoneAddressing, true)

    (0x88, "DEY", 2, AddressingMode::NoneAddressing, true)

    (0x49, "EOR", 2, AddressingMode::Immediate,      true)
    (0x45, "EOR", 3, AddressingMode::ZeroPage,       true)
    (0x55, "EOR", 4, AddressingMode::ZeroPageX,      true)
    (0x4D, "EOR", 4, AddressingMode::Absolute,       true)
    (0x5D, "EOR", 4, AddressingMode::AbsoluteX,      true)
    (0x59, "EOR", 4, AddressingMode::AbsoluteY,      true)
    (0x41, "EOR", 6, AddressingMode::IndirectX,      true)
    (0x51, "EOR", 5, AddressingMode::IndirectY,      true)

    (0xE6, "INC", 5, AddressingMode::ZeroPage,       true)
    (0xF6, "INC", 6, AddressingMode::ZeroPageX,      true)
    (0xEE, "INC", 6, AddressingMode::Absolute,       true)
    (0xFE, "INC", 7, AddressingMode::AbsoluteX,      true)

    (0xE8, "INX", 2, AddressingMode::NoneAddressing, true)

    (0xC8, "INY", 2, AddressingMode::NoneAddressing, true)

    (0x4C, "JMP", 3, AddressingMode::Absolute,       false)
    (0x6C, "JMP", 5, AddressingMode::Indirect,       false)

    (0x20, "JSR", 6, AddressingMode::Absolute,       false)

    (0xA9, "LDA", 2, AddressingMode::Immediate,      true)
    (0xA5, "LDA", 3, AddressingMode::ZeroPage,       true)
    (0xB5, "LDA", 4, AddressingMode::ZeroPageX,      true)
    (0xAD, "LDA", 4, AddressingMode::Absolute,       true)
    (0xBD, "LDA", 4, AddressingMode::AbsoluteX,      true)
    (0xB9, "LDA", 4, AddressingMode::AbsoluteY,      true)
    (0xA1, "LDA", 6, AddressingMode::IndirectX,      true)
    (0xB1, "LDA", 5, AddressingMode::IndirectY,      true)

    (0xA2, "LDX", 2, AddressingMode::Immediate,      true)
    (0xA6, "LDX", 3, AddressingMode::ZeroPage,       true)
    (0xB6, "LDX", 4, AddressingMode::ZeroPageY,       true)
    (0xAE, "LDX", 4, AddressingMode::Absolute,       true)
    (0xBE, "LDX", 4, AddressingMode::AbsoluteY,      true)

    (0xA0, "LDY", 2, AddressingMode::Immediate,      true)
    (0xA4, "LDY", 3, AddressingMode::ZeroPage,       true)
    (0xB4, "LDY", 4, AddressingMode::ZeroPageX,       true)
    (0xAC, "LDY", 4, AddressingMode::Absolute,       true)
    (0xBC, "LDY", 4, AddressingMode::AbsoluteX,      true)

    (0x4A, "LSR", 2, AddressingMode::NoneAddressing, true)
    (0x46, "LSR", 5, AddressingMode::ZeroPage,       true)
    (0x56, "LSR", 6, AddressingMode::ZeroPageX,      true)
    (0x4E, "LSR", 6, AddressingMode::Absolute,       true)
    (0x5E, "LSR", 7, AddressingMode::AbsoluteX,      true)

    (0xEA, "NOP", 1, AddressingMode::NoneAddressing, true)

    (0x09, "ORA", 2, AddressingMode::Immediate,      true)
    (0x05, "ORA", 3, AddressingMode::ZeroPage,       true)
    (0x15, "ORA", 4, AddressingMode::ZeroPageX,      true)
    (0x0D, "ORA", 4, AddressingMode::Absolute,       true)
    (0x1D, "ORA", 4, AddressingMode::AbsoluteX,      true)
    (0x19, "ORA", 4, AddressingMode::AbsoluteY,      true)
    (0x01, "ORA", 6, AddressingMode::IndirectX,      true)
    (0x11, "ORA", 5, AddressingMode::IndirectY,      true)

    (0x48, "PHA", 3, AddressingMode::NoneAddressing, true)

    (0x08, "PHP", 3, AddressingMode::NoneAddressing, true)

    (0x68, "PLA", 4, AddressingMode::NoneAddressing, true)

    (0x28, "PLP", 4, AddressingMode::NoneAddressing, true)

    (0x2A, "ROL", 2, AddressingMode::NoneAddressing, true)
    (0x26, "ROL", 5, AddressingMode::ZeroPage,       true)
    (0x36, "ROL", 6, AddressingMode::ZeroPageX,      true)
    (0x2E, "ROL", 6, AddressingMode::Absolute,       true)
    (0x3E, "ROL", 7, AddressingMode::AbsoluteX,      true)

    (0x6A, "ROR", 2, AddressingMode::NoneAddressing, true)
    (0x66, "ROR", 5, AddressingMode::ZeroPage,       true)
    (0x76, "ROR", 6, AddressingMode::ZeroPageX,      true)
    (0x6E, "ROR", 6, AddressingMode::Absolute,       true)
    (0x7E, "ROR", 7, AddressingMode::AbsoluteX,      true)

    (0x40, "RTI", 6, AddressingMode::NoneAddressing, true)

    (0x60, "RTS", 6, AddressingMode::NoneAddressing, true)

    (0xE9, "SBC", 2, AddressingMode::Immediate,      true)
    (0xE5, "SBC", 3, AddressingMode::ZeroPage,       true)
    (0xF5, "SBC", 4, AddressingMode::ZeroPageX,      true)
    (0xED, "SBC", 4, AddressingMode::Absolute,       true)
    (0xFD, "SBC", 4, AddressingMode::AbsoluteX,      true)
    (0xF9, "SBC", 4, AddressingMode::AbsoluteY,      true)
    (0xE1, "SBC", 6, AddressingMode::IndirectX,      true)
    (0xF1, "SBC", 5, AddressingMode::IndirectY,      true)

    (0x38, "SEC", 2, AddressingMode::NoneAddressing, true)

    (0xF8, "SED", 2, AddressingMode::NoneAddressing, true)

    (0x78, "SEI", 2, AddressingMode::NoneAddressing, true)

    (0x85, "STA", 3, AddressingMode::ZeroPage,       true)
    (0x95, "STA", 4, AddressingMode::ZeroPageX,      true)
    (0x8D, "STA", 4, AddressingMode::Absolute,       true)
    (0x9D, "STA", 5, AddressingMode::AbsoluteX,      true)
    (0x99, "STA", 5, AddressingMode::AbsoluteY,      true)
    (0x81, "STA", 6, AddressingMode::IndirectX,      true)
    (0x91, "STA", 6, AddressingMode::IndirectY,      true)

    (0x86, "STX", 3, AddressingMode::ZeroPage,       true)
    (0x96, "STX", 4, AddressingMode::ZeroPageY,      true)
    (0x8E, "STX", 4, AddressingMode::Absolute,       true)

    (0x84, "STY", 3, AddressingMode::ZeroPage,       true)
    (0x94, "STY", 4, AddressingMode::ZeroPageX,      true)
    (0x8C, "STY", 4, AddressingMode::Absolute,       true)

    (0xAA, "TAX", 2, AddressingMode::NoneAddressing, true)

    (0xA8, "TAY", 2, AddressingMode::NoneAddressing, true)

    (0xBA, "TSX", 2, AddressingMode::NoneAddressing, true)

    (0x8A, "TXA", 2, AddressingMode::NoneAddressing, true)

    (0x9A, "TXS", 2, AddressingMode::NoneAddressing, true)

    (0x98, "TYA", 2, AddressingMode::NoneAddressing, true)
}

impl crate::cpu::CPU {
    pub(super) fn adc(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult {
            address,
            page_crossed,
        } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        let (result, overflow) = self.register_a.overflowing_add(value);
        let (result, overflow_carry) = result.overflowing_add(self.get_carry_flag());

        self.set_carry_flag((overflow || overflow_carry) as u8);
        self.set_overflow_flag(
            ((result >> 7 != self.register_a >> 7) && (self.register_a >> 7 == value >> 7)) as u8,
        );

        self.register_a = result;
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        match mode {
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY | AddressingMode::IndirectY => {
                if page_crossed {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub(super) fn and(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, page_crossed } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        self.register_a = self.register_a & value;
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        match mode {
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY | AddressingMode::IndirectY => {
                if page_crossed {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub(super) fn asl(&mut self, mode: &AddressingMode) -> u8 {
        match mode {
            AddressingMode::NoneAddressing => {
                self.set_carry_flag(self.register_a >> 7);
                self.register_a = self.register_a.overflowing_shl(1).0;
                self.calc_zero_flag(self.register_a);
                self.calc_negative_flag(self.register_a);
            }
            mode => {
                let AddressResult { address, .. } = self.get_operand_address(mode);

                let value = self.bus.mem_read(address);
                self.set_carry_flag(value >> 7);

                let new_value = value.overflowing_shl(1).0;
                self.bus.mem_write(address, new_value);
                self.calc_zero_flag(new_value);
                self.calc_negative_flag(new_value);
            }
        }

        return 0;
    }

    pub(super) fn bcc(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);
        if self.get_carry_flag() == 0 {
            self.program_counter = address;
        }

        return 0;
    }

    pub(super) fn bcs(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_carry_flag() == 1 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn beq(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_zero_flag() == 1 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn bit(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);
        self.calc_zero_flag(value & self.register_a);
        self.set_overflow_flag((value & 0b0100_0000) >> 6);
        self.set_negative_flag((value & 0b1000_0000) >> 7);

        return 0;
    }

    pub(super) fn bmi(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_negative_flag() == 1 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn bne(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_zero_flag() == 0 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn bpl(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_negative_flag() == 0 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn bvc(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_overflow_flag() == 0 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn bvs(&mut self, mode: &AddressingMode) -> u8 {
        if self.get_overflow_flag() == 1 {
            let AddressResult { address, .. } = self.get_operand_address(mode);

            self.program_counter = address;
        } else {
            self.program_counter += 1
        }

        return 0;
    }

    pub(super) fn clc(&mut self) -> u8 {
        self.set_carry_flag(0);

        return 0;
    }

    pub(super) fn cld(&mut self) -> u8 {
        self.set_decimal_flag(0);

        return 0;
    }

    pub(super) fn cli(&mut self) -> u8 {
        self.set_interrupt_disable_flag(0);

        return 0;
    }

    pub(crate) fn clv(&mut self) -> u8 {
        self.set_overflow_flag(0);

        return 0;
    }

    pub(super) fn cmp(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        if self.register_a >= value {
            self.set_carry_flag(1)
        } else {
            self.set_carry_flag(0)
        }

        if self.register_a == value {
            self.set_zero_flag(1)
        } else {
            self.set_zero_flag(0)
        }

        if self.register_a.overflowing_sub(value).0 >> 7 == 1 {
            self.set_negative_flag(1)
        } else {
            self.set_negative_flag(0)
        }

        return 0;
    }

    pub(super) fn cpx(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        if self.register_x >= value {
            self.set_carry_flag(1)
        } else {
            self.set_carry_flag(0)
        }

        if self.register_x == value {
            self.set_zero_flag(1)
        } else {
            self.set_zero_flag(0)
        }

        if self.register_x.overflowing_sub(value).0 >> 7 == 1 {
            self.set_negative_flag(1)
        } else {
            self.set_negative_flag(0)
        }

        return 0;
    }

    pub(super) fn cpy(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        if self.register_y >= value {
            self.set_carry_flag(1)
        } else {
            self.set_carry_flag(0)
        }

        if self.register_y == value {
            self.set_zero_flag(1)
        } else {
            self.set_zero_flag(0)
        }

        if self.register_y.overflowing_sub(value).0 >> 7 == 1 {
            self.set_negative_flag(1)
        } else {
            self.set_negative_flag(0)
        }

        return 0;
    }

    pub(super) fn dec(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let mut value = self.bus.mem_read(address);
        value = value.overflowing_sub(1).0;
        self.bus.mem_write(address, value);

        self.calc_zero_flag(value);
        self.calc_negative_flag(value);

        return 0;
    }

    pub(super) fn dex(&mut self) -> u8 {
        self.register_x = self.register_x.overflowing_sub(1).0;

        self.calc_zero_flag(self.register_x);
        self.calc_negative_flag(self.register_x);

        return 0;
    }

    pub(super) fn dey(&mut self) -> u8 {
        self.register_y = self.register_y.overflowing_sub(1).0;

        self.calc_zero_flag(self.register_y);
        self.calc_negative_flag(self.register_y);

        return 0;
    }

    pub(super) fn eor(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        self.register_a = self.register_a ^ value;

        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn jmp(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        self.program_counter = address;

        return 0;
    }

    pub(super) fn jsr(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        self.stack_push_u16(self.program_counter.wrapping_sub(1));
        self.program_counter = address;

        return 0;
    }

    pub(super) fn inc(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let mut value = self.bus.mem_read(address);
        value = value.overflowing_add(1).0;
        self.bus.mem_write(address, value);

        self.calc_zero_flag(value);
        self.calc_negative_flag(value);

        return 0;
    }

    pub(super) fn inx(&mut self) -> u8 {
        self.register_x = self.register_x.overflowing_add(1).0;

        self.calc_zero_flag(self.register_x);
        self.calc_negative_flag(self.register_x);

        return 0;
    }

    pub(super) fn iny(&mut self) -> u8 {
        self.register_y = self.register_y.overflowing_add(1).0;

        self.calc_zero_flag(self.register_y);
        self.calc_negative_flag(self.register_y);

        return 0;
    }

    pub(super) fn lda(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        self.register_a = value;
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn ldx(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        self.register_x = value;
        self.calc_zero_flag(self.register_x);
        self.calc_negative_flag(self.register_x);

        return 0;
    }

    pub(super) fn ldy(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        self.register_y = value;
        self.calc_zero_flag(self.register_y);
        self.calc_negative_flag(self.register_y);

        return 0;
    }

    pub(super) fn lsr(&mut self, mode: &AddressingMode) -> u8 {
        match mode {
            AddressingMode::NoneAddressing => {
                self.set_carry_flag(self.register_a & 0b0000_0001);
                self.register_a = self.register_a.overflowing_shr(1).0;
                self.calc_zero_flag(self.register_a);
                self.calc_negative_flag(self.register_a);
            }
            mode => {
                let AddressResult { address, .. } = self.get_operand_address(mode);

                let value = self.bus.mem_read(address);
                self.set_carry_flag(value & 0b0000_0001);

                let new_value = value.overflowing_shr(1).0;
                self.bus.mem_write(address, new_value);
                self.calc_zero_flag(new_value);
                self.calc_negative_flag(new_value);
            }
        }

        return 0;
    }

    pub(super) fn ora(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        self.register_a = self.register_a | value;

        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn pha(&mut self) -> u8 {
        self.stack_push(self.register_a);

        return 0;
    }

    pub(super) fn php(&mut self) -> u8 {
        // When pushing status to stack B value on
        // destination should  be set to 1:
        // http://forum.6502.org/viewtopic.php?t=770
        self.stack_push(self.status | 0b0001_0000);

        return 0;
    }

    pub(super) fn pla(&mut self) -> u8 {
        self.register_a = self.stack_pop();
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn plp(&mut self) -> u8 {
        // In nes version this inctruction ignores bits 4 and 5
        // https://www.nesdev.org/wiki/Status_flags
        self.status = self.status & 0b0011_0000 | (self.stack_pop() & 0b1100_1111);

        return 0;
    }

    pub(super) fn rol(&mut self, mode: &AddressingMode) -> u8 {
        let old_carry = self.get_carry_flag();
        match mode {
            AddressingMode::NoneAddressing => {
                self.set_carry_flag(self.register_a >> 7);
                self.register_a = self.register_a.overflowing_shl(1).0;
                self.register_a += old_carry;

                self.calc_zero_flag(self.register_a);
                self.calc_negative_flag(self.register_a);
            }
            mode => {
                let AddressResult { address, .. } = self.get_operand_address(mode);

                let mut value = self.bus.mem_read(address);

                self.set_carry_flag(value >> 7);
                value = value.overflowing_shl(1).0;
                value += old_carry;

                self.bus.mem_write(address, value);
                self.calc_zero_flag(value);
                self.calc_negative_flag(value);
            }
        }

        return 0;
    }

    pub(super) fn ror(&mut self, mode: &AddressingMode) -> u8 {
        let old_carry = self.get_carry_flag();
        match mode {
            AddressingMode::NoneAddressing => {
                self.set_carry_flag(self.register_a & 1);
                self.register_a = self.register_a.overflowing_shr(1).0;
                self.register_a += old_carry << 7;

                self.calc_zero_flag(self.register_a);
                self.calc_negative_flag(self.register_a);
            }
            mode => {
                let AddressResult { address, .. } = self.get_operand_address(mode);

                let mut value = self.bus.mem_read(address);

                self.set_carry_flag(value & 1);
                value = value.overflowing_shr(1).0;
                value += old_carry << 7;

                self.bus.mem_write(address, value);
                self.calc_zero_flag(value);
                self.calc_negative_flag(value);
            }
        }

        return 0;
    }

    pub(super) fn rts(&mut self) -> u8 {
        self.program_counter = self.stack_pop_u16() + 1;

        return 0;
    }

    pub(super) fn tax(&mut self) -> u8 {
        self.register_x = self.register_a;
        self.calc_zero_flag(self.register_x);
        self.calc_negative_flag(self.register_x);

        return 0;
    }

    pub(super) fn tay(&mut self) -> u8 {
        self.register_y = self.register_a;
        self.calc_zero_flag(self.register_y);
        self.calc_negative_flag(self.register_y);

        return 0;
    }

    pub(super) fn tsx(&mut self) -> u8 {
        self.register_x = self.stack_pointer;
        self.calc_zero_flag(self.register_x);
        self.calc_negative_flag(self.register_x);

        return 0;
    }

    pub(super) fn txa(&mut self) -> u8 {
        self.register_a = self.register_x;
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn txs(&mut self) -> u8 {
        self.stack_pointer = self.register_x;

        return 0;
    }

    pub(super) fn tya(&mut self) -> u8 {
        self.register_a = self.register_y;
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn sbc(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        let value = self.bus.mem_read(address);

        let (result, overflow) = self.register_a.overflowing_sub(value);
        let (result, overflow_carry) = result.overflowing_sub(self.get_carry_flag() ^ 1);

        self.set_carry_flag(!(overflow || overflow_carry) as u8);

        self.set_overflow_flag(
            ((result >> 7 != self.register_a >> 7) && (self.register_a >> 7 != value >> 7)) as u8,
        );

        self.register_a = result;
        self.calc_zero_flag(self.register_a);
        self.calc_negative_flag(self.register_a);

        return 0;
    }

    pub(super) fn sec(&mut self) -> u8 {
        self.set_carry_flag(1);

        return 0;
    }

    pub(super) fn sed(&mut self) -> u8 {
        self.set_decimal_flag(1);

        return 0;
    }

    pub(super) fn sei(&mut self) -> u8 {
        self.set_interrupt_disable_flag(1);

        return 0;
    }

    pub(super) fn sta(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        self.bus.mem_write(address, self.register_a);

        return 0;
    }

    pub(super) fn stx(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        self.bus.mem_write(address, self.register_x);

        return 0;
    }

    pub(super) fn sty(&mut self, mode: &AddressingMode) -> u8 {
        let AddressResult { address, .. } = self.get_operand_address(mode);

        self.bus.mem_write(address, self.register_y);

        return 0;
    }
}
