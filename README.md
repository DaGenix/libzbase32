# libzbase32

libzbase32 is a `no_std` compatible crate that supports encoding and
decoding data in the z-base-32 format, as specified
[here](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt).

Z-base-32 is intended to be easier for a human to work with than regular
Base32 specified by [RFC 4658](https://datatracker.ietf.org/doc/html/rfc4648).

Some of the key differences:

* Z-base-32 a different alphabet ("ybndrfg8ejkmcpqxot1uwisza345h769") which
  consists of all lower-case letters (this library will accept lower-case
  or uppercase letters when decoding). The alphabet was chosen to make
  easier to use character appear more frequently in the output.

* Z-base-32 that the parties encoding and decoding z-base-32 values have
  some mechanism to agree on the length of the data. z-base-32 never
  includes padding characters (eg: "=") in order to keep the representation
  more compact.

* With Z-base-32, data lengths are specified in bits. This allows for more compact
  encodings. For example, in z-base-32, a 5 bit value can be encoded
  into a single character; while base32 would produce an
  8 character encoded value (of which 6 characters are padding bytes).

## Documentation

Modules documentation is available [here](https://docs.rs/libzbase32).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.
