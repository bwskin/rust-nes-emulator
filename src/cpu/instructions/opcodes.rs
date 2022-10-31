macro_rules! OPCODES {
    ($(
        ($code:expr, $instruction_name:expr, $cycles:expr, $addresing_mode:expr, $increment:expr)
    )*) => {
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        pub struct OpCode {
            // pub code: u8,
            pub instruction_name: String,
            pub cycles: u8,
            pub addresing_mode: AddressingMode,
            pub increment: bool
        }

        lazy_static! {
            pub static ref OPCODES: HashMap<u8, OpCode> = {
                let mut map = HashMap::new();
                $(
                    map.insert(
                        $code,
                        OpCode {
                            // code: $code,
                            instruction_name: String::from($instruction_name),
                            cycles: $cycles,
                            addresing_mode: $addresing_mode,
                            increment: $increment,
                        },
                    );
                )*
                map
            };
        }
    };
}

pub(crate) use OPCODES;
