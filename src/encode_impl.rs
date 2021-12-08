use crate::error::{
    input_buffer_doesnt_match_bits, invalid_quintet, output_buffer_doesnt_match_bits,
};
use crate::stateful_encoder::{
    octet_has_valid_trailing_bits, HaveQuintets, NeedOctets, NextQuintetResult, ProvideOctetResult,
};
use crate::tables::QUINTET_TO_CHARACTER;
use crate::util::{required_octets_buffer_len, required_quintets_buffer_len};
use crate::{UsageError, ZBase32Error};
use core::iter::Peekable;

enum OctetsToQuintetsIterState {
    Initial(NeedOctets),
    HaveQuintets(HaveQuintets),
}

struct OctetsToQuintetsIter<I>
where
    I: Iterator<Item = u8>,
{
    octet_iter: Peekable<I>,
    state: Option<OctetsToQuintetsIterState>,
}

impl<I> OctetsToQuintetsIter<I>
where
    I: Iterator<Item = u8>,
{
    fn new(octet_iter: I, need_octets: NeedOctets) -> OctetsToQuintetsIter<I> {
        OctetsToQuintetsIter {
            octet_iter: octet_iter.peekable(),
            state: Some(OctetsToQuintetsIterState::Initial(need_octets)),
        }
    }
}

fn refill<I>(
    octet_iter: &mut Peekable<I>,
    mut need_octets: NeedOctets,
) -> Result<Option<OctetsToQuintetsIterState>, ZBase32Error>
where
    I: Iterator<Item = u8>,
{
    loop {
        let octet = octet_iter.next().unwrap();
        let last_octet = octet_iter.peek().is_none();
        match need_octets.provide_octet(octet, last_octet)? {
            ProvideOctetResult::NeedOctets(need_more) => need_octets = need_more,
            ProvideOctetResult::HaveQuintets(have_quintets) => {
                return Ok(Some(OctetsToQuintetsIterState::HaveQuintets(have_quintets)))
            }
        }
    }
}

impl<I> Iterator for OctetsToQuintetsIter<I>
where
    I: Iterator<Item = u8>,
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
                OctetsToQuintetsIterState::Initial(need_octets) => {
                    if self.octet_iter.peek().is_some() {
                        match refill(&mut self.octet_iter, need_octets) {
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
                OctetsToQuintetsIterState::HaveQuintets(have_quintets) => {
                    match have_quintets.next_quintet() {
                        NextQuintetResult::Quintet(quintet, have_quintets) => {
                            self.state =
                                Some(OctetsToQuintetsIterState::HaveQuintets(have_quintets));
                            return Some(Ok(quintet));
                        }
                        NextQuintetResult::NeedOctets(need_octets) => {
                            match refill(&mut self.octet_iter, need_octets) {
                                Ok(new_state) => self.state = new_state,
                                Err(err) => return Some(Err(err)),
                            }
                        }
                        NextQuintetResult::Complete => {}
                    }
                }
            }
        }
    }
}

/// Convert a quintet integer value (such as "0") to its character
/// value (such as "y").
pub fn quintet_to_character(quintet: u8) -> Result<u8, UsageError> {
    if quintet as usize > QUINTET_TO_CHARACTER.len() {
        return Err(invalid_quintet());
    }
    Ok(QUINTET_TO_CHARACTER[quintet as usize])
}

fn calc_last_octet_bits(bits: u64) -> Option<u8> {
    if bits == 0 {
        None
    } else {
        match bits % 8 {
            0 => Some(8),
            x => Some(x as u8),
        }
    }
}

/// Determine if the last octet is valid, given the number of bits to encode
pub fn is_last_octet_valid(bits: u64, octet: u8) -> bool {
    if let Some(last_octet_bits) = calc_last_octet_bits(bits) {
        octet_has_valid_trailing_bits(last_octet_bits, octet)
    } else {
        false
    }
}

/// Convert a buffer of octets (bytes) into a buffer of quintet values (ie, integers between 0-31).
///
/// The length of `in_octets` must match the value returned by
/// [`required_octets_buffer_len`] and the length of `out_quintets`
/// must match the value returned by [`required_quintets_buffer_len`].
pub fn octets_to_quintets(
    in_octets: &[u8],
    out_quintets: &mut [u8],
    bits: u64,
) -> Result<(), ZBase32Error> {
    if in_octets.len() != required_octets_buffer_len(bits)? {
        return Err(input_buffer_doesnt_match_bits().into());
    }
    if out_quintets.len() != required_quintets_buffer_len(bits)? {
        return Err(output_buffer_doesnt_match_bits().into());
    }
    let last_octet_bits = if let Some(x) = calc_last_octet_bits(bits) {
        x
    } else {
        return Ok(());
    };

    let quintet_iter = OctetsToQuintetsIter::new(
        in_octets.iter().map(|&x| x),
        NeedOctets::new(last_octet_bits),
    );

    for (out_quintet, in_quintet) in out_quintets.iter_mut().zip(quintet_iter) {
        *out_quintet = in_quintet?;
    }

    Ok(())
}

/// Encode a buffer of octets (bytes) to a buffer of characters.
///
/// This method is the low-level equivalent of the [`encode`] method.
/// Unlike that method, this method does not allocate and is usable
/// in `no_std` mode.
///
/// This method is equivalent to first using [`octets_to_quintets`] to
/// convert all octet (byte) values to quintet integers and then using
/// [`quintet_to_character`] to convert those values to character values. This method,
/// however, may be more efficient as it is able to do both operations
/// together in a single pass over the input buffer.
///
/// The length of `in_octets` must match the value returned by
/// [`required_octets_buffer_len`] and the length of `out_characters`
/// must match the value returned by [`required_quintets_buffer_len`].
pub fn encode_slices(
    in_octets: &[u8],
    out_characters: &mut [u8],
    bits: u64,
) -> Result<(), ZBase32Error> {
    if in_octets.len() != required_octets_buffer_len(bits)? {
        return Err(input_buffer_doesnt_match_bits().into());
    }
    if out_characters.len() != required_quintets_buffer_len(bits)? {
        return Err(output_buffer_doesnt_match_bits().into());
    }
    let last_octet_bits = if let Some(x) = calc_last_octet_bits(bits) {
        x
    } else {
        return Ok(());
    };

    let quintet_iter = OctetsToQuintetsIter::new(
        in_octets.iter().map(|&x| x),
        NeedOctets::new(last_octet_bits),
    );

    for (out_quintet, in_quintet) in out_characters.iter_mut().zip(quintet_iter) {
        *out_quintet = quintet_to_character(in_quintet?)?;
    }

    Ok(())
}

/// Encode a slice of octets (bytes) to a [`String`].
///
/// The output characters will be appended to `output`.
///
/// The length of `input` must match the value returned by
/// [`required_octets_buffer_len`]. The `output` buffer will be
/// extended by to accommodate the output.
///
/// This method is not available in `no_std` mode.
#[cfg(feature = "std")]
pub fn encode(input: &[u8], output: &mut String, bits: u64) -> Result<(), ZBase32Error> {
    if input.len() != required_octets_buffer_len(bits)? {
        return Err(input_buffer_doesnt_match_bits().into());
    }
    let last_octet_bits = if let Some(x) = calc_last_octet_bits(bits) {
        x
    } else {
        return Ok(());
    };

    let needed_quintets = required_quintets_buffer_len(bits)?;
    output.reserve(needed_quintets);

    let quintet_iter =
        OctetsToQuintetsIter::new(input.iter().map(|&x| x), NeedOctets::new(last_octet_bits));

    for quintet in quintet_iter {
        output.push(quintet_to_character(quintet?)? as char);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::encode;
    use crate::test_data::{TestCase, RANDOM_TEST_DATA, STANDARD_TEST_DATA};

    fn run_tests(test_cases: &[TestCase]) {
        let mut buffer = String::new();
        for test in test_cases {
            buffer.clear();
            encode(test.unencoded, &mut buffer, test.bits).unwrap();
            assert_eq!(&buffer, test.encoded);
        }
    }

    #[test]
    fn test_encode_standard() {
        run_tests(STANDARD_TEST_DATA);
    }

    #[test]
    fn test_encode_random() {
        run_tests(RANDOM_TEST_DATA);
    }
}
