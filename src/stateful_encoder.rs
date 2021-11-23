use crate::error::trailing_nonzero_bits;
use crate::ZBase32Error;

pub struct NeedOctets {
    octet_buffer: [u8; 5],
    pos: u8,
    last_octet_bits: u8,
}

pub enum ProvideOctetResult {
    NeedOctets(NeedOctets),
    HaveQuintets(HaveQuintets),
}

pub fn octet_has_valid_trailing_bits(last_octet_bits: u8, octet: u8) -> bool {
    let trailing_bits_mask = 0xffu8.checked_shr(last_octet_bits as u32).unwrap_or(0);
    octet & trailing_bits_mask == 0
}

impl NeedOctets {
    // NOTE: panics if `last_octet_bits` is invalid. This would need to be changed
    // if this ever became a public API!
    pub fn new(last_octet_bits: u8) -> NeedOctets {
        assert!(last_octet_bits != 0 && last_octet_bits <= 8);
        NeedOctets {
            octet_buffer: [0u8; 5],
            pos: 0,
            last_octet_bits,
        }
    }

    pub fn provide_octet(
        mut self,
        octet: u8,
        last_octet: bool,
    ) -> Result<ProvideOctetResult, ZBase32Error> {
        if last_octet {
            if !octet_has_valid_trailing_bits(self.last_octet_bits, octet) {
                return Err(trailing_nonzero_bits());
            }
        }

        self.octet_buffer[self.pos as usize] = octet;
        self.pos += 1;

        if self.pos == 5 || last_octet {
            let mut quintet_buffer = 0;
            for count in 0..5 {
                quintet_buffer |= (self.octet_buffer[count] as u64) << (4 - count) * 8;
            }

            let quintet_buffer_bits =
                (self.pos - 1) * 8 + if last_octet { self.last_octet_bits } else { 8 };
            let remaining_quintets = (quintet_buffer_bits + 4) / 5;

            return Ok(ProvideOctetResult::HaveQuintets(HaveQuintets {
                quintet_buffer,
                remaining_quintets,
                last_octet_bits: self.last_octet_bits,
                completed: last_octet,
            }));
        } else {
            Ok(ProvideOctetResult::NeedOctets(self))
        }
    }
}

pub struct HaveQuintets {
    quintet_buffer: u64,
    remaining_quintets: u8,
    last_octet_bits: u8,
    completed: bool,
}

pub enum NextQuintetResult {
    Quintet(u8, HaveQuintets),
    NeedOctets(NeedOctets),
    Complete,
}

impl HaveQuintets {
    pub fn next_quintet(mut self) -> NextQuintetResult {
        if self.remaining_quintets == 0 {
            return if self.completed {
                NextQuintetResult::Complete
            } else {
                NextQuintetResult::NeedOctets(NeedOctets {
                    octet_buffer: [0u8; 5],
                    pos: 0,
                    last_octet_bits: self.last_octet_bits,
                })
            };
        }

        let val = ((self.quintet_buffer >> 35) as u8) & 0x1f;
        self.quintet_buffer = self.quintet_buffer << 5;
        self.remaining_quintets -= 1;

        NextQuintetResult::Quintet(val, self)
    }
}
