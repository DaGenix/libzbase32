//! # libzbase32
//!
//! libzbase32 is a `no_std` compatible crate that supports encoding and
//! decoding data in the z-base-32 format, as specified
//! [here](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt).
//!
//! Z-base-32 is intended to be easier for a human to work with than regular
//! Base32 specified by [RFC 4658](https://datatracker.ietf.org/doc/html/rfc4648).
//!
//! Some of the key differences:
//!
//! * Z-base-32 a different alphabet ("ybndrfg8ejkmcpqxot1uwisza345h769") which
//!   consists of all lower-case letters (this library will accept lower-case
//!   or uppercase letters when decoding). The alphabet was chosen to make
//!   easier to use character appear more frequently in the output.
//!
//! * Z-base-32 that the parties encoding and decoding z-base-32 values have
//!   some mechanism to agree on the length of the data. z-base-32 never
//!   includes padding characters (eg: "=") in order to keep the representation
//!   more compact.
//!
//! * With Z-base-32, data lengths are specified in bits. This allows for more compact
//!   encodings. For example, in z-base-32, a 5 bit value can be encoded
//!   into a single character; while base32 would produce an
//!   8 character encoded value (of which 6 characters are padding bytes).
//!
//! ## Usage
//!
//! There are two APIs that can be used - the high-level API and the low-level API.
//! The high-level API is a little more convenient to use and should
//! generally be used when possible - but is unavailable in no_std mode. The
//! low-level API allows for no-allocation operation as well as two-step operations
//! in which transformations between octets <-> quintets and quintets <-> characters
//! are separate operations - which can be useful for specialized use cases.
//!
//! When using either API, its important to remember that z-base-32 is big-endian
//! oriented. As such, if you encode a single bit, z-base-32 will encode
//! the _highest bit_ of the input byte.
//!
//! When encoding or decoding, if the input value includes non-zero bits past
//! the number of bits specified in the operation, an Error be returned. For example,
//! encoding a single bit for the input value 0x01 will fail.
//!
//! Most fallible operations return an Error value of the same type, [`ZBase32Error`].
//! This is a mostly opaque type that only allows you to differentiate between an
//! error in the input value or an error in using the interfaces. More information
//! about the cause of the error can be retrieved by using the
//! [`Debug::fmt`](std::fmt::Debug::fmt) or [`Display::fmt`](std::fmt::Display::fmt)
//! functions.
//!
//! ## High-level API
//!
//! The high-level API consists of two encoding functions
//! [`encode_full_bytes`] and [`encode`]; and two decoding functions
//! [`decode_full_bytes`] and [`decode`].
//!
//! [`encode_full_bytes`] and [`decode_full_bytes`] are simple to use
//! when you know that the unencoded bytestring is a whole number of
//! bytes.  [`encode`] and [`decode`] can also handle numbers of
//! unencoded bits that are not divisible by 8, and can also append
//! their results to existing variables.
//!
//! Example:
//!
//! ```
//! use libzbase32::{ZBase32Error, encode, encode_full_bytes, decode};
//!
//! # fn main() {
//! const DATA: &'static [u8] = &[0, 44, 55, 128];
//!
//! let full_bytes_encoded = encode_full_bytes(DATA);
//! assert_eq!(&full_bytes_encoded, "yysdxyy");
//!
//! let full_bytes_decoded = decode_full_bytes(full_bytes_encoded);
//! assert_eq!(&full_bytes_decoded, DATA);
//!
//! let mut encoded = String::new();
//! encode(DATA, &mut encoded, 25).expect("Encoding failed!");
//!
//! assert_eq!(&encoded, "yysdx");
//!
//! let mut decoded = Vec::new();
//! decode(&encoded, &mut decoded, 25).expect("Decoding failed!");
//!
//! assert_eq!(&decoded, DATA);
//! # }
//! ```
//!
//! ## Low-level API
//!
//! The low-level API is found in the [`low_level_encode`] and [`low_level_decode`] modules.
//!
//! Unlike the high-level API:
//!
//! * The low-level API does not allocate.
//!
//! * Unlike The high-level API, all input and output types are `&[u8]` and its up
//!   to the caller to differentiate between arrays of bytes (octets), quintets, or
//!   ASCII character values.
//!
//! * The low-level API supports two-step operations. Eg, when encoding, the low-level API allows
//!   users to first transform the input from octets in quintet values (integers
//!   in the range 0-31) and then later transform from quintet values into
//!   encoded characters. This can be handy for specialized applications.
//!
//! * Single-step operations are also supported via the
//!   [`encode_slices`](low_level_encode::encode_slices) and
//!   [`decode_slices`](low_level_decode::decode_slices) which function similarly
//!   to the functions in the high-level API - but require the caller to setup
//!   an appropriate output buffer.
//!
//! ## No_std
//!
//! No_std mode may be activated by disabling the "std" feature. In this
//! mode, only the low-level interfaces are available.
//!
//! ## License
//
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <https://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!   <https://opensource.org/licenses/MIT>)
//!
//! at your option.

#![cfg_attr(not(feature = "std"), no_std)]

mod decode_impl;
mod encode_impl;
mod error;
mod stateful_decoder;
mod stateful_encoder;
mod tables;
#[cfg(test)]
mod test_data;
mod util;

pub use error::{InputErrorCause, UsageError, UsageErrorCause, ZBase32Error};

#[cfg(feature = "std")]
pub use decode_impl::decode;

#[cfg(feature = "std")]
pub use encode_impl::encode;

pub mod low_level_decode {
    //! Low-level decoding functionality
    pub use crate::decode_impl::{
        character_to_quintet, decode_slices, is_last_quintet_valid, quintets_to_octets,
    };
    pub use crate::util::required_octets_buffer_len;
}

pub mod low_level_encode {
    //! Low-level encoding functionality
    pub use crate::encode_impl::{
        encode_slices, is_last_octet_valid, octets_to_quintets, quintet_to_character,
    };
    pub use crate::util::required_quintets_buffer_len;
}
