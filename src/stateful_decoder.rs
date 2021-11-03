use crate::error::{zbase32_error, ZBase32Error, ZBase32ErrorInfo};

pub struct NeedQuintets {
    quintet_buffer: [u8; 8],
    pos: u8,
    last_quintet_bits: u8,
}

pub enum ProvideQuintetResult {
    NeedQuintets(NeedQuintets),
    HaveOctets(HaveOctets),
}

impl NeedQuintets {
    pub fn new(last_quintet_bits: u8) -> Result<NeedQuintets, ZBase32Error> {
        assert!(last_quintet_bits != 0 && last_quintet_bits <= 5);
        Ok(NeedQuintets {
            quintet_buffer: [0u8; 8],
            pos: 0,
            last_quintet_bits,
        })
    }

    pub fn provide_quintet(
        mut self,
        quintet: u8,
        last_quintet: bool,
    ) -> Result<ProvideQuintetResult, ZBase32Error> {
        if quintet > 31 {
            return Err(zbase32_error(ZBase32ErrorInfo::InvalidQuintet));
        }

        if last_quintet {
            let trailing_bits_mask = 0x1fu8
                .checked_shr(self.last_quintet_bits as u32)
                .unwrap_or(0);
            if quintet & trailing_bits_mask != 0 {
                return Err(zbase32_error(ZBase32ErrorInfo::TrailingNonZeroBits));
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
