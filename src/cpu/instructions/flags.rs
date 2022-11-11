use paste::paste;

macro_rules! flag {
    ($name:ident, $mask:expr) => {
        paste! {
            #[allow(dead_code)]
            pub(super) fn [< get_ $name _flag >] (&self) -> u8 {
                self.get_flag($mask)
            }

            #[allow(dead_code)]
            pub(super) fn [< set_ $name _flag >] (&mut self, value: u8) {
                self.set_flag($mask, value)
            }
        }
    };
}

#[allow(dead_code)] // !TODO: Handle unused flags
impl crate::cpu::CPU {
    flag! { carry,             0b0000_0001 }
    flag! { zero,              0b0000_0010 }
    flag! { interrupt_disable, 0b0000_0100 }
    flag! { decimal,           0b0000_1000 }
    flag! { overflow,          0b0100_0000 }
    flag! { negative,          0b1000_0000 }

    pub(super) fn calc_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.set_zero_flag(1)
        } else {
            self.set_zero_flag(0)
        }
    }

    pub(super) fn calc_negative_flag(&mut self, result: u8) {
        if (result & 0b1000_0000) >> 7 == 1 {
            self.set_negative_flag(1)
        } else {
            self.set_negative_flag(0)
        }
    }

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
