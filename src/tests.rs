#![allow(dead_code)]

use super::cpu::CPU;
use super::bus::TestBus;
use serde::Deserialize;
use paste::paste;

const TESTS_PATH: &str = "src/tests/v1";

#[derive(Deserialize, Debug)]
struct RamEntry {
    addr: u16,
    value: u8,
}

#[derive(Deserialize, Debug)]
struct State {
    #[serde(rename = "pc")]
    program_counter: u16,
    #[serde(rename = "s")]
    stack_pointer: u8,
    #[serde(rename = "a")]
    register_a: u8,
    #[serde(rename = "x")]
    register_x: u8,
    #[serde(rename = "y")]
    register_y: u8,
    #[serde(rename = "p")]
    status: u8,
    ram: Vec<RamEntry>,
}

#[derive(Deserialize, Debug)]
struct TestCase {
    name: String,
    #[serde(rename = "initial")]
    init: State,
    #[serde(rename = "final")]
    result: State,
}

impl TestCase {
    fn run(&self) -> bool {
        let bus = TestBus::new();
        let mut cpu = CPU::new(bus);
        cpu.program_counter = self.init.program_counter;
        cpu.stack_pointer = self.init.stack_pointer;
        cpu.register_a = self.init.register_a;
        cpu.register_x = self.init.register_x;
        cpu.register_y = self.init.register_y;
        cpu.status = self.init.status;

        for ram_entry in self.init.ram.iter() {
            cpu.bus.mem_write(ram_entry.addr, ram_entry.value);
        }

        cpu.next();

        let mut passed = true;

        if cpu.program_counter != self.result.program_counter {
            print!(
                "Program counter invalid: {:04x} should be {:04x}; ",
                cpu.program_counter, self.result.program_counter
            );
            passed = false;
        };
        if cpu.stack_pointer != self.result.stack_pointer {
            print!("Stack pointer invalid: {:02x}; ", cpu.stack_pointer);
            passed = false;
        };
        if cpu.register_a != self.result.register_a {
            print!("Register A invalid: {:02x}; ", cpu.register_a);
            passed = false;
        };
        if cpu.register_x != self.result.register_x {
            print!("Register X invalid: {:02x}; ", cpu.register_x);
            passed = false;
        };
        if cpu.register_y != self.result.register_y {
            print!("Register Y invalid: {:02x}; ", cpu.register_y);
            passed = false;
        };
        if cpu.status != self.result.status {
            print!("Status register invalid: {:08b}; ", cpu.status);
            passed = false;
        };

        for ram_entry in self.result.ram.iter() {
            let value = cpu.bus.mem_read(ram_entry.addr);
            if value != ram_entry.value {
                print!(
                    "Memory at address 0x{:04x} invalid: {:04x} should be {:04x}; ",
                    ram_entry.addr, value, ram_entry.value
                );
                passed = false;
            }
        }

        if !passed {
            println!("On case {}", &self.name);
        }

        passed
    }
}

macro_rules! test_opcode {
    ($opcode:expr, $instruction_name:expr) => {
        paste! {
            #[allow(non_snake_case)]
            #[test]
            fn [< test_ $instruction_name _ $opcode >]() {
                match std::fs::read_to_string(format!("{}/{:02x}.json", TESTS_PATH, $opcode)) {
                    Ok(json) => {
                        let test_cases: Vec<TestCase> = serde_json::from_str(&json).unwrap();

                        for test_case in test_cases.iter() {
                            if test_case.run() {
                                return;
                            }
                        }
                    },
                    Err(_) => panic!("Test failed. Check test stdout")
                }
            }
        }
    }
}

test_opcode!(0x69, "ADC");
test_opcode!(0x65, "ADC");
test_opcode!(0x75, "ADC");
test_opcode!(0x6D, "ADC");
test_opcode!(0x7D, "ADC");
test_opcode!(0x79, "ADC");
test_opcode!(0x61, "ADC");
test_opcode!(0x71, "ADC");

test_opcode!(0x29, "AND");
test_opcode!(0x25, "AND");
test_opcode!(0x35, "AND");
test_opcode!(0x2D, "AND");
test_opcode!(0x3D, "AND");
test_opcode!(0x39, "AND");
test_opcode!(0x21, "AND");
test_opcode!(0x31, "AND");

test_opcode!(0x0A, "ASL");
test_opcode!(0x06, "ASL");
test_opcode!(0x16, "ASL");
test_opcode!(0x0E, "ASL");
test_opcode!(0x1E, "ASL");

test_opcode!(0x90, "BCC");

test_opcode!(0xB0, "BCS");

test_opcode!(0xF0, "BEQ");

test_opcode!(0x24, "BIT");
test_opcode!(0x2C, "BIT");

test_opcode!(0x30, "BMI");

test_opcode!(0xD0, "BNE");

test_opcode!(0x10, "BPL");

test_opcode!(0x00, "BRK");

test_opcode!(0x50, "BVC");

test_opcode!(0x70, "BVS");

test_opcode!(0x18, "CLC");

test_opcode!(0xD8, "CLD");

test_opcode!(0x58, "CLI");

test_opcode!(0xB8, "CLV");

test_opcode!(0xC9, "CMP");
test_opcode!(0xC5, "CMP");
test_opcode!(0xD5, "CMP");
test_opcode!(0xCD, "CMP");
test_opcode!(0xDD, "CMP");
test_opcode!(0xD9, "CMP");
test_opcode!(0xC1, "CMP");
test_opcode!(0xD1, "CMP");

test_opcode!(0xE0, "CPX");
test_opcode!(0xE4, "CPX");
test_opcode!(0xEC, "CPX");

test_opcode!(0xC0, "CPY");
test_opcode!(0xC4, "CPY");
test_opcode!(0xCC, "CPY");

test_opcode!(0xC6, "DEC");
test_opcode!(0xD6, "DEC");
test_opcode!(0xCE, "DEC");
test_opcode!(0xDE, "DEC");

test_opcode!(0xCA, "DEX");

test_opcode!(0x88, "DEY");

test_opcode!(0x49, "EOR");
test_opcode!(0x45, "EOR");
test_opcode!(0x55, "EOR");
test_opcode!(0x4D, "EOR");
test_opcode!(0x5D, "EOR");
test_opcode!(0x59, "EOR");
test_opcode!(0x41, "EOR");
test_opcode!(0x51, "EOR");

test_opcode!(0xE6, "INC");
test_opcode!(0xF6, "INC");
test_opcode!(0xEE, "INC");
test_opcode!(0xFE, "INC");

test_opcode!(0xE8, "INX");

test_opcode!(0xC8, "INY");

test_opcode!(0x4C, "JMP");
test_opcode!(0x6C, "JMP");

test_opcode!(0x20, "JSR");

test_opcode!(0xA9, "LDA");
test_opcode!(0xA5, "LDA");
test_opcode!(0xB5, "LDA");
test_opcode!(0xAD, "LDA");
test_opcode!(0xBD, "LDA");
test_opcode!(0xB9, "LDA");
test_opcode!(0xA1, "LDA");
test_opcode!(0xB1, "LDA");

test_opcode!(0xA2, "LDX");
test_opcode!(0xA6, "LDX");
test_opcode!(0xB6, "LDX");
test_opcode!(0xAE, "LDX");
test_opcode!(0xBE, "LDX");

test_opcode!(0xA0, "LDY");
test_opcode!(0xA4, "LDY");
test_opcode!(0xB4, "LDY");
test_opcode!(0xAC, "LDY");
test_opcode!(0xBC, "LDY");

test_opcode!(0x4A, "LSR");
test_opcode!(0x46, "LSR");
test_opcode!(0x56, "LSR");
test_opcode!(0x4E, "LSR");
test_opcode!(0x5E, "LSR");

test_opcode!(0xEA, "NOP");

test_opcode!(0x09, "ORA");
test_opcode!(0x05, "ORA");
test_opcode!(0x15, "ORA");
test_opcode!(0x0D, "ORA");
test_opcode!(0x1D, "ORA");
test_opcode!(0x19, "ORA");
test_opcode!(0x01, "ORA");
test_opcode!(0x11, "ORA");

test_opcode!(0x48, "PHA");

test_opcode!(0x08, "PHP");

test_opcode!(0x68, "PLA");

test_opcode!(0x28, "PLP");

test_opcode!(0x2A, "ROL");
test_opcode!(0x26, "ROL");
test_opcode!(0x36, "ROL");
test_opcode!(0x2E, "ROL");
test_opcode!(0x3E, "ROL");

test_opcode!(0x6A, "ROR");
test_opcode!(0x66, "ROR");
test_opcode!(0x76, "ROR");
test_opcode!(0x6E, "ROR");
test_opcode!(0x7E, "ROR");

test_opcode!(0x40, "RTI");

test_opcode!(0x60, "RTS");

test_opcode!(0xE9, "SBC");
test_opcode!(0xE5, "SBC");
test_opcode!(0xF5, "SBC");
test_opcode!(0xED, "SBC");
test_opcode!(0xFD, "SBC");
test_opcode!(0xF9, "SBC");
test_opcode!(0xE1, "SBC");
test_opcode!(0xF1, "SBC");

test_opcode!(0x38, "SEC");

test_opcode!(0xF8, "SED");

test_opcode!(0x78, "SEI");

test_opcode!(0x85, "STA");
test_opcode!(0x95, "STA");
test_opcode!(0x8D, "STA");
test_opcode!(0x9D, "STA");
test_opcode!(0x99, "STA");
test_opcode!(0x81, "STA");
test_opcode!(0x91, "STA");

test_opcode!(0x86, "STX");
test_opcode!(0x96, "STX");
test_opcode!(0x8E, "STX");

test_opcode!(0x84, "STY");
test_opcode!(0x94, "STY");
test_opcode!(0x8C, "STY");

test_opcode!(0xAA, "TAX");

test_opcode!(0xA8, "TAY");

test_opcode!(0xBA, "TSX");

test_opcode!(0x8A, "TXA");

test_opcode!(0x9A, "TXS");

test_opcode!(0x98, "TYA");
