use crate::error::{decoding_trailing_nonzero_bits, invalid_quintet};
use crate::InputError;

pub struct NeedQuintets {
    quintet_buffer: [u8; 8],
    pos: u8,
    last_quintet_bits: u8,
}

pub enum ProvideQuintetResult {
    NeedQuintets(NeedQuintets),
    HaveOctets(HaveOctets),
}

pub fn quintet_has_valid_trailing_bits(last_quintet_bits: u8, quintet: u8) -> bool {
    let trailing_bits_mask = 0x1fu8.checked_shr(last_quintet_bits as u32).unwrap_or(0);
    quintet & trailing_bits_mask == 0
}

impl NeedQuintets {
    // NOTE: panics if `last_quintet_bits` is invalid. This would need to be changed
    // if this ever became a public API!
    pub fn new(last_quintet_bits: u8) -> NeedQuintets {
        assert!(last_quintet_bits != 0 && last_quintet_bits <= 5);
        NeedQuintets {
            quintet_buffer: [0u8; 8],
            pos: 0,
            last_quintet_bits,
        }
    }

    pub fn provide_quintet(
        mut self,
        quintet: u8,
        last_quintet: bool,
    ) -> Result<ProvideQuintetResult, InputError> {
        if quintet > 31 {
            return Err(invalid_quintet().into());
        }

        if last_quintet {
            if !quintet_has_valid_trailing_bits(self.last_quintet_bits, quintet) {
                return Err(decoding_trailing_nonzero_bits());
            }
        }

        self.quintet_buffer[self.pos as usize] = quintet;
        self.pos += 1;

        if self.pos == 8 || last_quintet {
            let mut octet_buffer = 0;
            for count in 0..8 {
                octet_buffer |= (self.quintet_buffer[count] as u64) << (7 - count) * 5;
            }

            let octet_buffer_bits = (self.pos - 1) * 5
                + if last_quintet {
                    self.last_quintet_bits
                } else {
                    5
                };
            let remaining_octets = (octet_buffer_bits + 7) / 8;

            return Ok(ProvideQuintetResult::HaveOctets(HaveOctets {
                octet_buffer,
                remaining_octets,
                last_quintet_bits: self.last_quintet_bits,
                completed: last_quintet,
            }));
        } else {
            Ok(ProvideQuintetResult::NeedQuintets(self))
        }
    }
}

pub struct HaveOctets {
    octet_buffer: u64,
    remaining_octets: u8,
    last_quintet_bits: u8,
    completed: bool,
}

pub enum NextOctetResult {
    Octet(u8, HaveOctets),
    NeedQuintets(NeedQuintets),
    Complete,
}

impl HaveOctets {
    pub fn next_octet(mut self) -> NextOctetResult {
        if self.remaining_octets == 0 {
            return if self.completed {
                NextOctetResult::Complete
            } else {
                NextOctetResult::NeedQuintets(NeedQuintets {
                    quintet_buffer: [0u8; 8],
                    pos: 0,
                    last_quintet_bits: self.last_quintet_bits,
                })
            };
        }

        let val = (self.octet_buffer >> 32) as u8;
        self.octet_buffer = self.octet_buffer << 8;
        self.remaining_octets -= 1;

        NextOctetResult::Octet(val, self)
    }
}
