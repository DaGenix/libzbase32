use crate::error::{
    input_buffer_doesnt_match_bits, invalid_character, output_buffer_doesnt_match_bits,
};
use crate::stateful_decoder::{
    quintet_has_valid_trailing_bits, HaveOctets, NeedQuintets, NextOctetResult,
    ProvideQuintetResult,
};
use crate::tables::{CHARACTER_MIN_VALUE, CHARACTER_TO_QUINTET};
use crate::util::{required_octets_buffer_len, required_quintets_buffer_len};
use crate::ZBase32Error;
use core::iter::Peekable;

enum QuintetsToOctetsIterState {
    Initial(NeedQuintets),
    HaveOctets(HaveOctets),
}

struct QuintetsToOctetsIter<I>
where
    I: Iterator<Item = Result<u8, ZBase32Error>>,
{
    quintet_iter: Peekable<I>,
    state: Option<QuintetsToOctetsIterState>,
}

impl<I> QuintetsToOctetsIter<I>
where
    I: Iterator<Item = Result<u8, ZBase32Error>>,
{
    fn new(quintet_iter: I, need_quintets: NeedQuintets) -> QuintetsToOctetsIter<I> {
        QuintetsToOctetsIter {
            quintet_iter: quintet_iter.peekable(),
            state: Some(QuintetsToOctetsIterState::Initial(need_quintets)),
        }
    }
}

fn refill<I>(
    quintet_iter: &mut Peekable<I>,
    mut need_quintets: NeedQuintets,
) -> Result<Option<QuintetsToOctetsIterState>, ZBase32Error>
where
    I: Iterator<Item = Result<u8, ZBase32Error>>,
{
    loop {
        let quintet = quintet_iter.next().unwrap()?;
        let last_quintet = quintet_iter.peek().is_none();
        match need_quintets.provide_quintet(quintet, last_quintet)? {
            ProvideQuintetResult::NeedQuintets(need_more) => need_quintets = need_more,
            ProvideQuintetResult::HaveOctets(have_octets) => {
                return Ok(Some(QuintetsToOctetsIterState::HaveOctets(have_octets)))
            }
        }
    }
}

impl<I> Iterator for QuintetsToOctetsIter<I>
where
    I: Iterator<Item = Result<u8, ZBase32Error>>,
{
    type Item = Result<u8, ZBase32Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            if self.state.is_none() {
                return None;
            }

            match self.state.take().unwrap() {
                // The first time we are called, we start in the initial state
                // and have to read some quintets from the underlying iterator.
                // After doing this, we'll loop and never end up in this state
                // again.
                QuintetsToOctetsIterState::Initial(need_quintets) => {
                    if self.quintet_iter.peek().is_some() {
                        match refill(&mut self.quintet_iter, need_quintets) {
                            Ok(new_state) => self.state = new_state,
                            Err(err) => return Some(Err(err)),
                        }
                    } else {
                        // Do nothing - this means that the input iterator was empty -
                        // which shouldn't actually be possible.
                    }
                }

                // If we are in the HaveOctets state, it means we can try to return
                // already processed octet values. If we run out of octets, we will
                // need to use the refill method to process more quintets into
                // octets. If we hit the end of the input, we'll leave self.state
                // as None as a signal to complete.
                QuintetsToOctetsIterState::HaveOctets(have_octets) => {
                    match have_octets.next_octet() {
                        NextOctetResult::Octet(octet, have_octets) => {
                            self.state = Some(QuintetsToOctetsIterState::HaveOctets(have_octets));
                            return Some(Ok(octet));
                        }
                        NextOctetResult::NeedQuintets(need_quintets) => {
                            match refill(&mut self.quintet_iter, need_quintets) {
                                Ok(new_state) => self.state = new_state,
                                Err(err) => return Some(Err(err)),
                            }
                        }
                        NextOctetResult::Complete => {}
                    }
                }
            }
        }
    }
}

/// Convert a character code value (such as "y") to its integer
/// value (such as 0).
pub fn character_to_quintet(character: u8) -> Result<u8, ZBase32Error> {
    if character < CHARACTER_MIN_VALUE {
        return Err(invalid_character());
    } else if (character - CHARACTER_MIN_VALUE) as usize >= CHARACTER_TO_QUINTET.len() {
        return Err(invalid_character());
    }
    let val = CHARACTER_TO_QUINTET[(character - CHARACTER_MIN_VALUE) as usize];
    if val == 255 {
        return Err(invalid_character());
    }
    Ok(val)
}

fn calc_last_quintet_bits(bits: u64) -> Option<u8> {
    if bits == 0 {
        None
    } else {
        match bits % 5 {
            0 => Some(5),
            x => Some(x as u8),
        }
    }
}

/// Determine if the last quintet is, given the number of bits to decode
pub fn is_last_quintet_valid(bits: u64, quintet: u8) -> bool {
    if let Some(last_quintet_bits) = calc_last_quintet_bits(bits) {
        quintet <= 31 && quintet_has_valid_trailing_bits(last_quintet_bits, quintet)
    } else {
        false
    }
}

/// Convert a buffer of quintet integer values (ie, integers of the
/// range 0-31) into a buffer of octet (byte) values.
///
/// The length of `in_quintets` must match the value returned by
/// [`required_quintets_buffer_len`] and the length of `out_octets`
/// must match the value returned by [`required_octets_buffer_len`].
pub fn quintets_to_octets(
    in_quintets: &[u8],
    out_octets: &mut [u8],
    bits: u64,
) -> Result<(), ZBase32Error> {
    if in_quintets.len() != required_quintets_buffer_len(bits)? {
        return Err(input_buffer_doesnt_match_bits().into());
    }
    if out_octets.len() != required_octets_buffer_len(bits)? {
        return Err(output_buffer_doesnt_match_bits().into());
    }
    let last_quintet_bits = if let Some(x) = calc_last_quintet_bits(bits) {
        x
    } else {
        return Ok(());
    };

    let octet_iter = QuintetsToOctetsIter::new(
        in_quintets.iter().map(|&x| Ok(x)),
        NeedQuintets::new(last_quintet_bits),
    );

    for (out, next_octet) in out_octets.iter_mut().zip(octet_iter) {
        *out = next_octet?;
    }

    Ok(())
}

/// Decode a buffer of characters to a buffer of octets (bytes).
///
/// This method is the low-level equivalent of the [`decode`] method.
/// Unlike that method, this method does not allocate and is usable
/// in `no_std` mode.
///
/// This method is equivalent to first using [`character_to_quintet`] to
/// convert all character values to quintet integers and then using
/// [`quintets_to_octets`] to convert to octet (byte) values. This method,
/// however, may be more efficient as it is able to do both operations
/// together in a single pass over the input buffer.
///
/// The length of `in_characters` must match the value returned by
/// [`required_quintets_buffer_len`] and the length of `out_octets`
/// must match the value returned by [`required_octets_buffer_len`].
pub fn decode_slices(
    in_characters: &[u8],
    out_octets: &mut [u8],
    bits: u64,
) -> Result<(), ZBase32Error> {
    if in_characters.len() != required_quintets_buffer_len(bits)? {
        return Err(input_buffer_doesnt_match_bits().into());
    }
    if out_octets.len() != required_octets_buffer_len(bits)? {
        return Err(output_buffer_doesnt_match_bits().into());
    }
    let last_quintet_bits = if let Some(x) = calc_last_quintet_bits(bits) {
        x
    } else {
        return Ok(());
    };

    let octet_iter = QuintetsToOctetsIter::new(
        in_characters.iter().map(|&x| character_to_quintet(x)),
        NeedQuintets::new(last_quintet_bits),
    );

    for (out, next_octet) in out_octets.iter_mut().zip(octet_iter) {
        *out = next_octet?;
    }

    Ok(())
}

/// Decode a slice of characters to a [`Vec`] of octets (bytes).
///
/// The output octets will be appended to `output`.
///
/// The length of `input` must match the value returned by
/// [`required_quintets_buffer_len`]. The `output` buffer will be
/// extended by to accommodate the output.
///
/// This method is not available in `no_std` mode.
#[cfg(feature = "std")]
pub fn decode(input: &str, output: &mut Vec<u8>, bits: u64) -> Result<(), ZBase32Error> {
    if input.len() != required_quintets_buffer_len(bits)? {
        return Err(input_buffer_doesnt_match_bits().into());
    }
    let last_quintet_bits = if let Some(x) = calc_last_quintet_bits(bits) {
        x
    } else {
        return Ok(());
    };

    let needed_octets = required_octets_buffer_len(bits)?;
    let start = output.len();
    output.extend(std::iter::repeat(0).take(needed_octets));
    let output_buff = &mut output[start..];

    let octet_iter = QuintetsToOctetsIter::new(
        input.as_bytes().iter().map(|&x| character_to_quintet(x)),
        NeedQuintets::new(last_quintet_bits),
    );

    for (out, next_octet) in output_buff.iter_mut().zip(octet_iter) {
        *out = next_octet?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use super::decode;
    use super::decode_slices;
    use super::required_octets_buffer_len;
    use crate::test_data::{TestCase, RANDOM_TEST_DATA, STANDARD_TEST_DATA};

    #[cfg(feature = "std")]
    fn run_tests(test_cases: &[TestCase]) {
        let mut buffer = Vec::new();
        for test in test_cases {
            buffer.clear();
            decode(test.encoded, &mut buffer, test.bits).unwrap();
            assert_eq!(&buffer[..], test.unencoded);
        }
    }

    fn run_low_level_tests(test_cases: &[TestCase]) {
        // the largest sample in src/test_data.rs has 128 bits.
        let mut buffer: [u8; 256] = [0; 256];
        for test in test_cases {
            let outsz:usize = required_octets_buffer_len(test.bits).unwrap();
            decode_slices(test.encoded.as_bytes(), &mut buffer[..outsz], test.bits).unwrap();
            assert_eq!(&buffer[..outsz], test.unencoded);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_decode_standard() {
        run_tests(STANDARD_TEST_DATA);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_decode_random() {
        run_tests(RANDOM_TEST_DATA);
    }

    #[test]
    fn test_decode_low_level_standard() {
        run_low_level_tests(STANDARD_TEST_DATA);
    }

    #[test]
    fn test_decode_low_level_random() {
        run_low_level_tests(RANDOM_TEST_DATA);
    }

    #[test]
    fn test_decode_non_zbase32_characters() {
        let badcharacters = [ "0", "L", "💩", " ", "!", "_", "2", "v"];
        let mut out:[u8; 1] = [0];
        for character in badcharacters {
            assert!(decode_slices(character.as_bytes(), &mut out, 5).is_err(),
                    "'{}' is not valid zbase32 character", character);
        }
    }

    #[test]
    fn test_decode_non_full_byte_strings() {
        // Should really also test different length encodings and bit
        // boundaries, not just two chars in, one octet out. But it's
        // clumsy to do this in no_std (without allocation), so just a
        // simple test for now.

        // these two-character strings have some bits set in the
        // trailing bits after the last full octet:
        let badstrings = [ "99", "y9", "on", "yt", "zb"];
        let mut out:[u8;1] = [0];
        for string in badstrings {
            assert!(decode_slices(string.as_bytes(), &mut out, 8).is_err(),
                    "'{}' should have produced bits beyond bit 8, should not have decoded, got {:#?}",
                    string, out);
        }
    }
}
