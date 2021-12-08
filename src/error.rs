use core::fmt::{Debug, Display, Formatter};

enum InputErrorType {
    InvalidCharacter,
    DecodingTrailingNonzeroBits,
}

pub struct InputErrorCause {
    typ: InputErrorType,
}

impl Debug for InputErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.typ {
            InputErrorType::InvalidCharacter => write!(f, "Invalid character found in input."),
            InputErrorType::DecodingTrailingNonzeroBits => {
                write!(f, "Trailing non-zero bits found in decode input.")
            }
        }
    }
}

impl Display for InputErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

enum UsageErrorType {
    InputBufferDoesntMatchBits,
    OutputBufferDoesntMatchBits,
    BitsOverflow,
    InvalidQuintet,
    EncodingTrailingNonzeroBits,
}

pub struct UsageErrorCause {
    typ: UsageErrorType,
}

impl Debug for UsageErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.typ {
            UsageErrorType::InputBufferDoesntMatchBits => {
                write!(
                    f,
                    "The input buffer size doesn't agree with the provided bits value"
                )
            }
            UsageErrorType::OutputBufferDoesntMatchBits => {
                write!(
                    f,
                    "The output buffer size doesn't agree with the provided bits value"
                )
            }
            UsageErrorType::BitsOverflow => {
                write!(f, "The value for bits was too large for the platform usize")
            }
            UsageErrorType::InvalidQuintet => write!(f, "Invalid quintet value found in input."),
            UsageErrorType::EncodingTrailingNonzeroBits => {
                write!(
                    f,
                    "Trailing non-zero bits found in data to encode. \
                    Check that the value of the 'bits' parameter matches the data \
                    to be encoded."
                )
            }
        }
    }
}

impl Display for UsageErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Common error type used by all fallible operations where bad input could be the cause
///
/// By design, this type is mostly opaque - with the exception
/// that its possible to differentiate between errors with input
/// data and other types of errors. The [`Debug`] or [`Display`]
/// implementations can be used to format a more specific error
/// message.
pub enum InputError {
    /// An InputError indicates that an input array contained an invalid
    /// value. For example, a non-zbase32 character being passed to one of
    /// the decode methods.
    InputError(InputErrorCause),

    /// A UsageError indicates an error outside of an invalid input value.
    UsageError(UsageErrorCause),
}

impl Debug for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InputError::InputError(cause) => write!(f, "Input Error: {}", cause),
            InputError::UsageError(cause) => write!(f, "Usage Error: {}", cause),
        }
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InputError {}

impl From<UsageError> for InputError {
    fn from(err: UsageError) -> Self {
        InputError::UsageError(err.0)
    }
}

/// A UsageError indicates an error outside of an invalid input value.
///
/// By design, this type is mostly opaque. The [`Debug`] or [`Display`]
/// implementations can be used to format a more specific error
/// message.
pub struct UsageError(pub UsageErrorCause);

impl Debug for UsageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Usage Error: {}", self.0)
    }
}

impl Display for UsageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UsageError {}

pub const fn invalid_character() -> InputError {
    InputError::InputError(InputErrorCause {
        typ: InputErrorType::InvalidCharacter,
    })
}

pub const fn invalid_quintet() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::InvalidQuintet,
    })
}

pub const fn decoding_trailing_nonzero_bits() -> InputError {
    InputError::InputError(InputErrorCause {
        typ: InputErrorType::DecodingTrailingNonzeroBits,
    })
}

pub const fn input_buffer_doesnt_match_bits() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::InputBufferDoesntMatchBits,
    })
}

pub const fn output_buffer_doesnt_match_bits() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::OutputBufferDoesntMatchBits,
    })
}

pub const fn bits_overflow() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::BitsOverflow,
    })
}

pub const fn encoding_trailing_nonzero_bits() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::EncodingTrailingNonzeroBits,
    })
}
