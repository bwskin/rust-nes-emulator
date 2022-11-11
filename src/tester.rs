use rust_nes_emu::cpu::{instructions::OPCODES, CPU};
use rust_nes_emu::bus::{TestBus, Bus};
use serde::Deserialize;

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

fn test(test_case: &TestCase) -> bool {
    let mut bus = TestBus::new();

    for ram_entry in test_case.init.ram.iter() {
        bus.mem_write(ram_entry.addr, ram_entry.value);
    }

    let mut cpu = CPU::new(bus);

    cpu.program_counter = test_case.init.program_counter;
    cpu.stack_pointer = test_case.init.stack_pointer;
    cpu.register_a = test_case.init.register_a;
    cpu.register_x = test_case.init.register_x;
    cpu.register_y = test_case.init.register_y;
    cpu.status = test_case.init.status;


    cpu.next();

    let mut passed = true;

    if cpu.program_counter != test_case.result.program_counter {
        print!("Program counter invalid: {:04x} should be {:04x}; ", cpu.program_counter, test_case.result.program_counter);
        passed = false;
    };
    if cpu.stack_pointer != test_case.result.stack_pointer {
        print!("Stack pointer invalid: {:02x}; ", cpu.stack_pointer);
        passed = false;
    };
    if cpu.register_a != test_case.result.register_a {
        print!("Register A invalid: {:02x}; ", cpu.register_a);
        passed = false;
    };
    if cpu.register_x != test_case.result.register_x {
        print!("Register X invalid: {:02x}; ", cpu.register_x);
        passed = false;
    };
    if cpu.register_y != test_case.result.register_y {
        print!("Register Y invalid: {:02x}; ", cpu.register_y);
        passed = false;
    };
    if cpu.status != test_case.result.status {
        print!("Status register invalid: {:08b}; ", cpu.status);
        passed = false;
    };

    for ram_entry in test_case.result.ram.iter() {
        let value = cpu.bus.mem_read(ram_entry.addr);
        if value != ram_entry.value {
            print!("Memory at address 0x{:04x} invalid: {:04x} should be {:04x}; ", ram_entry.addr, value, ram_entry.value);
            passed = false;
        }
    }

    if !passed {
        println!("On case {}", &test_case.name);
    }

    passed
}

fn test_opcode(opcode: u8) {
    print!("TESTING: 0x{:02x} -> ", opcode);

    match std::fs::read_to_string(format!("./test_jsons/v1/{:02x}.json", opcode)) {
        Ok(json) => {
            let test_cases: Vec<TestCase> = serde_json::from_str(&json).unwrap();

            for test_case in test_cases.iter() {
                if !test(test_case) {
                    return;
                }
            }

            println!("OK");
        },
        Err(_) => { println!("ERROR"); }
    }
}

fn main() {
    for opcode_entry in OPCODES.iter() {
        if *opcode_entry.0 != 0x00 && *opcode_entry.0 != 0x40 {
            test_opcode(*opcode_entry.0);
        }
    }
}

