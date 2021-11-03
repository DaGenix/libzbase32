use core::fmt::{Debug, Display, Formatter};

/// Common error type used by all fallible operations
///
/// By design, this type is mostly opaque - with the exception
/// that its possible to differentiate between errors with input
/// data and other types of errors. The [`Debug`] or [`Display`]
/// implementations can be used to format a more specific error
/// message.
pub struct ZBase32Error {
    error_info: ZBase32ErrorInfo,
}

/// Provides a set of error categories for ZBase32Error values
pub enum ZBase32ErrorType {
    /// An InputError indicates that an input array contained an invalid
    /// value. For example, a non-zbase32 character being passed to one of
    /// the decode methods.
    InputError,

    /// A UsageError indicates an error outside of an invalid input value.
    UsageError,
}

pub enum ZBase32ErrorInfo {
    InvalidCharacter,
    InvalidQuintet,
    TrailingNonZeroBits,
    InputBufferDoesntMatchBits,
    OutputBufferDoesntMatchBits,
    BitsOverflow,
}

impl ZBase32Error {
    /// Get the type of the error
    ///
    /// The type is either [`ZBase32ErrorType::InputError`] to
    /// indicate that something was wrong with the input or
    /// [`ZBase32ErrorType::UsageError`] to indicate that an API
    /// was used incorrectly.
    pub fn error_type(&self) -> ZBase32ErrorType {
        match self.error_info {
            ZBase32ErrorInfo::InvalidCharacter => ZBase32ErrorType::InputError,
            ZBase32ErrorInfo::InvalidQuintet => ZBase32ErrorType::InputError,
            ZBase32ErrorInfo::TrailingNonZeroBits => ZBase32ErrorType::InputError,
            ZBase32ErrorInfo::InputBufferDoesntMatchBits => ZBase32ErrorType::UsageError,
            ZBase32ErrorInfo::OutputBufferDoesntMatchBits => ZBase32ErrorType::UsageError,
            ZBase32ErrorInfo::BitsOverflow => ZBase32ErrorType::UsageError,
        }
    }
}

impl Debug for ZBase32Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.error_info {
            ZBase32ErrorInfo::InvalidCharacter => write!(f, "Invalid character found in input."),
            ZBase32ErrorInfo::InvalidQuintet => write!(f, "Invalid quintet value found in input."),
            ZBase32ErrorInfo::TrailingNonZeroBits => {
                write!(f, "Trailing non-zero bits found in input.")
            }
            ZBase32ErrorInfo::InputBufferDoesntMatchBits => {
                write!(
                    f,
                    "The input buffer size doesn't agree with the provided bits value"
                )
            }
            ZBase32ErrorInfo::OutputBufferDoesntMatchBits => {
                write!(
                    f,
                    "The output buffer size doesn't agree with the provided bits value"
                )
            }
            ZBase32ErrorInfo::BitsOverflow => {
                write!(f, "The value for bits was too large for the platform usize")
            }
        }
    }
}

impl Display for ZBase32Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ZBase32Error {}

pub const fn zbase32_error(error_info: ZBase32ErrorInfo) -> ZBase32Error {
    ZBase32Error { error_info }
}
