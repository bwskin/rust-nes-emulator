// Memory management implemented here
pub mod memory;
// Instructions and opcodes implemented here
pub mod instructions;
use instructions::OPCODES;

use crate::bus::Bus;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: u8,
    pub program_counter: u16,
    pub jmp_compat: bool,
    pub bus: Box<dyn Bus>
}

#[allow(dead_code)]
pub struct InstructionResult {
    end_of_program: bool,
    cycles: u8,
}

impl CPU {
    pub fn new(bus: Box<dyn Bus>) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0,
            status: 0,
            program_counter: 0,
            jmp_compat: true,
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0;
        self.status = 0;

        self.program_counter = self.bus.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            let result = self.next();
            if result.end_of_program { break };
        }
    }

    pub fn run_with_callback(&mut self, mut callback: impl FnMut(&mut CPU, u8) -> ()) {
        callback(self, 0);
        loop {
            let result = self.next();
            if result.end_of_program {
                break;
            };
            callback(self, result.cycles);
        }
    }

    pub fn next(&mut self) -> InstructionResult {
        let opcode = self.pop_read();

        let ins = OPCODES
            .get(&opcode)
            .unwrap_or_else(|| panic!("opcode {:X} is not implemented", opcode));

        let mut end_of_program = false;

        let additional_cycles = match ins.instruction_name.as_str() {
            "ADC" => self.adc(&ins.addresing_mode),
            "AND" => self.and(&ins.addresing_mode),
            "ASL" => self.asl(&ins.addresing_mode),
            "BCC" => self.bcc(&ins.addresing_mode),
            "BCS" => self.bcs(&ins.addresing_mode),
            "BEQ" => self.beq(&ins.addresing_mode),
            "BIT" => self.bit(&ins.addresing_mode),
            "BMI" => self.bmi(&ins.addresing_mode),
            "BNE" => self.bne(&ins.addresing_mode),
            "BPL" => self.bpl(&ins.addresing_mode),
            "BRK" => {
                end_of_program = true;
                0
            } // !TODO: implement BRK
            "BVC" => self.bvc(&ins.addresing_mode),
            "BVS" => self.bvs(&ins.addresing_mode),
            "CLC" => self.clc(),
            "CLD" => self.cld(),
            "CLI" => self.cli(),
            "CLV" => self.clv(),
            "CMP" => self.cmp(&ins.addresing_mode),
            "CPX" => self.cpx(&ins.addresing_mode),
            "CPY" => self.cpy(&ins.addresing_mode),
            "DEC" => self.dec(&ins.addresing_mode),
            "DEX" => self.dex(),
            "DEY" => self.dey(),
            "EOR" => self.eor(&ins.addresing_mode),
            "INC" => self.inc(&ins.addresing_mode),
            "INX" => self.inx(),
            "INY" => self.iny(),
            "JMP" => self.jmp(&ins.addresing_mode),
            "JSR" => self.jsr(&ins.addresing_mode),
            "LDA" => self.lda(&ins.addresing_mode),
            "LDX" => self.ldx(&ins.addresing_mode),
            "LDY" => self.ldy(&ins.addresing_mode),
            "LSR" => self.lsr(&ins.addresing_mode),
            "NOP" => 0, // !TODO: think of better handling this
            "ORA" => self.ora(&ins.addresing_mode),
            "PHA" => self.pha(),
            "PHP" => self.php(),
            "PLA" => self.pla(),
            "PLP" => self.plp(),
            "ROL" => self.rol(&ins.addresing_mode),
            "ROR" => self.ror(&ins.addresing_mode),
            "RTI" => 0, // !RODO: implement RTI
            "RTS" => self.rts(),
            "SBC" => self.sbc(&ins.addresing_mode),
            "SEC" => self.sec(),
            "SED" => self.sed(),
            "SEI" => self.sei(),
            "STA" => self.sta(&ins.addresing_mode),
            "STX" => self.stx(&ins.addresing_mode),
            "STY" => self.sty(&ins.addresing_mode),
            "TAX" => self.tax(),
            "TAY" => self.tay(),
            "TSX" => self.tsx(),
            "TXA" => self.txa(),
            "TXS" => self.txs(),
            "TYA" => self.tya(),
            _ => panic!("instruction {} is not implemented", ins.instruction_name),
        };

        let cycles = additional_cycles + ins.cycles;

        InstructionResult {
            end_of_program,
            cycles,
        }
    }
}
