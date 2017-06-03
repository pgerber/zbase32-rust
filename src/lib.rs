//! Encoding and decoding of zbase32.
//!
//! This is an implementation of the human-oriented base-32 encoding called
//! [zbase32](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt).

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

#![cfg_attr(feature="clippy", allow(inline_always))]
#![cfg_attr(feature = "clippy", warn(cast_possible_wrap))]
#![cfg_attr(feature = "clippy", warn(cast_precision_loss))]
#![cfg_attr(feature = "clippy", warn(cast_sign_loss))]
#![cfg_attr(feature = "clippy", warn(empty_enum))]
#![cfg_attr(feature = "clippy", warn(enum_glob_use))]
#![cfg_attr(feature = "clippy", warn(float_arithmetic))]
#![cfg_attr(feature = "clippy", warn(items_after_statements))]
#![cfg_attr(feature = "clippy", warn(if_not_else))]
#![cfg_attr(feature = "clippy", deny(mem_forget))]
#![cfg_attr(feature = "clippy", warn(mut_mut))]
#![cfg_attr(feature = "clippy", warn(nonminimal_bool))]
#![cfg_attr(feature = "clippy", warn(option_map_unwrap_or))]
#![cfg_attr(feature = "clippy", warn(option_map_unwrap_or_else))]
#![cfg_attr(feature = "clippy", warn(option_unwrap_used))]
#![cfg_attr(feature = "clippy", warn(print_stdout))]
#![cfg_attr(feature = "clippy", warn(result_unwrap_used))]
#![cfg_attr(feature = "clippy", deny(unicode_not_nfc))]
#![cfg_attr(feature = "clippy", deny(unseparated_literal_suffix))]
#![cfg_attr(feature = "clippy", deny(used_underscore_binding))]
#![cfg_attr(feature = "clippy", deny(wrong_pub_self_convention))]
#![cfg_attr(feature = "clippy", deny(wrong_self_convention))]


/// Alphabet used by zbase32
pub const ALPHABET: &[u8; 32] = b"ybndrfg8ejkmcpqxot1uwisza345h769";

enum ZType {
    Digit(u8),
    Whitespace,
    Err(&'static str),
}

impl ZType {
    fn is_valid(&self) -> bool {
        if let ZType::Err(_) = *self {
            false
        } else {
            true
        }
    }
}

#[inline]
fn value_of_digit(digit: u8) -> ZType {
    match digit {
        b'y' => ZType::Digit(0x00),
        b'b' => ZType::Digit(0x01),
        b'n' => ZType::Digit(0x02),
        b'd' => ZType::Digit(0x03),
        b'r' => ZType::Digit(0x04),
        b'f' => ZType::Digit(0x05),
        b'g' => ZType::Digit(0x06),
        b'8' => ZType::Digit(0x07),
        b'e' => ZType::Digit(0x08),
        b'j' => ZType::Digit(0x09),
        b'k' => ZType::Digit(0x0a),
        b'm' => ZType::Digit(0x0b),
        b'c' => ZType::Digit(0x0c),
        b'p' => ZType::Digit(0x0d),
        b'q' => ZType::Digit(0x0e),
        b'x' => ZType::Digit(0x0f),
        b'o' => ZType::Digit(0x10),
        b't' => ZType::Digit(0x11),
        b'1' => ZType::Digit(0x12),
        b'u' => ZType::Digit(0x13),
        b'w' => ZType::Digit(0x14),
        b'i' => ZType::Digit(0x15),
        b's' => ZType::Digit(0x16),
        b'z' => ZType::Digit(0x17),
        b'a' => ZType::Digit(0x18),
        b'3' => ZType::Digit(0x19),
        b'4' => ZType::Digit(0x1a),
        b'5' => ZType::Digit(0x1b),
        b'h' => ZType::Digit(0x1c),
        b'7' => ZType::Digit(0x1d),
        b'6' => ZType::Digit(0x1e),
        b'9' => ZType::Digit(0x1f),
        b' ' | b'\t' | b'\n' => ZType::Whitespace,
        _ => ZType::Err("not a zbase32 digit"),
    }
}

/// Decode first N `bits` of given zbase32 encoded data
///
/// # Panic
///
/// Panics if `zbase32` decoded is shorter than N `bits`.
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert_eq!(zbase32::decode(b"o", 1).unwrap(), &[0x80]);
/// ```
#[inline]
pub fn decode(zbase32: &[u8], bits: u64) -> Result<Vec<u8>, &'static str> {
    decode_internal(zbase32, Some(bits))
}

fn decode_internal(zbase32: &[u8], bits: Option<u64>) -> Result<Vec<u8>, &'static str> {
    let capacity = bits.map_or(0, |bits| {
        if bits % 8 == 0 {
            bits / 8
        } else {
            bits / 8 + 1
        }
    }) as usize;
    let mut result = Vec::with_capacity(capacity);

    let mut bits_remaining = bits;
    let mut buffer_size: u8 = 0;
    let mut buffer: u16 = !0;
    for digit in zbase32 {
        let value = match value_of_digit(*digit) {
            ZType::Digit(digit) => digit,
            ZType::Whitespace => continue,
            ZType::Err(e) => return Err(e),
        };
        buffer = (buffer << 5) | value as u16;
        buffer_size += 5;
        if let Some(bits_remaining) = bits_remaining {
            if bits_remaining < 8 && buffer_size as u64 >= bits_remaining {
                break;
            }
        }
        if buffer_size >= 8 {
            let byte = (buffer >> (buffer_size - 8)) as u8;
            result.push(byte);
            if let Some(mut r) = bits_remaining.as_mut() {
                *r -= 8;
            };
            buffer_size -= 8;
        }
    }
    let remaining = bits_remaining.unwrap_or(buffer_size as u64);
    assert!(remaining <= buffer_size as u64, "zbase64 slice too short");
    if remaining > 0 {
        let trim_right = buffer_size - remaining as u8;
        buffer >>= trim_right;
        buffer_size -= trim_right;
        let byte = (buffer << (8_u8 - buffer_size)) as u8;
        result.push(byte);
    }
    if let Some(_) = bits {
        debug_assert_eq!(capacity, result.len())
    };
    Ok(result)
}

/// Decode given zbase32 encoded string
///
/// Just like `decode` but doesn't allow decoding with bit precision.
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert_eq!(zbase32::decode_full_bytes(b"qb1ze3m1").unwrap(), b"peter");
/// ```
#[inline]
pub fn decode_full_bytes(zbase: &[u8]) -> Result<Vec<u8>, &'static str> {
    decode_internal(zbase, None)
}

/// Decode first N `bits` of given zbase32 encoded string
///
/// # Panic
///
/// Panics if `zbase32` decoded is shorter than N `bits`.
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert_eq!(zbase32::decode_str("o", 1).unwrap(), &[0x80]);
/// ```
#[inline]
pub fn decode_str(zbase32: &str, bits: u64) -> Result<Vec<u8>, &'static str> {
    decode_internal(zbase32.as_bytes(), Some(bits))
}

/// Decode given zbase32 encoded string
///
/// Just like `decode_str` but doesn't allow decoding with bit precision.
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert_eq!(zbase32::decode_full_bytes_str("qb1ze3m1").unwrap(), b"peter");
/// ```
#[inline]
pub fn decode_full_bytes_str(zbase32: &str) -> Result<Vec<u8>, &'static str> {
    decode_full_bytes(zbase32.as_bytes())
}

/// Encode first N `bits` with zbase32.
///
/// # Panics
///
/// Panics if `data` is shorter than N `bits`.
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert_eq!(zbase32::encode(b"testdata", 64), "qt1zg7drcf4gn");
/// ```
///
pub fn encode(data: &[u8], bits: u64) -> String {
    assert!(data.len() as u64 * 8 >= bits, "slice too short");
    let capacity = if bits % 5 == 0 {
        bits / 5
    } else {
        bits / 5 + 1
    } as usize;
    let mut result = Vec::with_capacity(capacity);

    let mut bits_remaining = bits;
    let mut bit_offset: u8 = 0;
    let mut remaining = data;
    let mut buffer = match data.len() {
        0 => !0 /* unused */,
        1 => (data[0] as u16) << 8,
        _ => {
            remaining = &data[2..];
            (data[0] as u16) << 8 | data[1] as u16
        }
    };

    while bits_remaining > 0 {
        let unused_bits = 5_u64.saturating_sub(bits_remaining);
        let index = (buffer >> (16 - 5 - bit_offset + unused_bits as u8) << unused_bits) & 0x1f;
        result.push(ALPHABET[index as usize]);

        bit_offset += 5;
        bits_remaining -= 5 - unused_bits;

        if bit_offset >= 8 {
            match remaining.split_first() {
                Some((first, others)) => {
                    buffer = buffer << 8 | *first as u16;
                    remaining = others;
                }
                None => {
                    buffer <<= 8;
                }
            }
            bit_offset -= 8;
        }
    }

    debug_assert_eq!(capacity, result.len());
    unsafe { String::from_utf8_unchecked(result) }
}

/// Encode full bytes using zbase32.
///
/// Just like `encode` but doesn't allow encoding with bit precision.
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// let data = "Just an arbitrary sentence.";
/// assert_eq!(zbase32::encode_full_bytes(data.as_bytes()),
///            "jj4zg7bycfznyam1cjwzehubqjh1yh5fp34gk5udcwzy");
/// ```
#[inline]
pub fn encode_full_bytes(data: &[u8]) -> String {
    encode(data, data.len() as u64 * 8)
}

/// Check if `data` is valid zbase32 encoded bytes
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert!(zbase32::validate(b"y1"));
/// assert!(!zbase32::validate(b"A"));
/// ```
pub fn validate(data: &[u8]) -> bool {
    data.iter().all(|i| value_of_digit(*i).is_valid())
}

/// Check if `data` is valid zbase32 encoded string
///
/// # Examples
///
/// ```
/// use zbase32;
///
/// assert!(zbase32::validate_str("y1"));
/// assert!(!zbase32::validate_str("A"));
/// ```
#[inline(always)]
pub fn validate_str(data: &str) -> bool {
    validate(data.as_bytes())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable")]
    extern crate test;
    extern crate rand;

    use super::*;
    use tests::rand::Rng;

    const INVALID_TEST_DATA: &[&str] = &["ybndrfg8ejkmcpqxot1uwisza345H769", "bnℕe", "uv", "l"];
    const SPACE_CHARS: &[u8] = b" \t\n";
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const TEST_DATA: &[(u64, &str, &[u8])] = &[
        (0,   "",       &[]),
        (1,   "y",      &[0x00]),
        (1,   "o",      &[0x80]),
        (2,   "e",      &[0x40]),
        (2,   "a",      &[0xc0]),
        (8,   "yy",     &[0x00]),
        (10,  "yy",     &[0x00, 0x00]),
        (10,  "on",     &[0x80, 0x80]),
        (20,  "tqre",   &[0x8b, 0x88, 0x80]),
        (24,  "6n9hq",  &[0xf0, 0xbf, 0xc7]),
        (24,  "4t7ye",  &[0xd4, 0x7a, 0x04]),
        (30,  "6im5sd", &[0xf5, 0x57, 0xbb, 0x0c]),
        (160, "ybndrfg8ejkmcpqxot1uwisza345h769", &[0x00, 0x44, 0x32, 0x14, 0xc7, 0x42, 0x54, 0xb6,
                                                    0x35, 0xcf, 0x84, 0x65, 0x3a, 0x56, 0xd7, 0xc6,
                                                    0x75, 0xbe, 0x77, 0xdf])
    ];
    #[cfg(feature = "unstable")]
    const ONE_MIB: usize = 1048576;

    #[test]
    fn test_decode() {
        for &(bits, zbase32, data) in TEST_DATA {
            assert_eq!(decode(zbase32.as_bytes(), bits).unwrap(), data);
        }
    }

    #[test]
    fn test_decode_with_whitespace() {
        for &(bits, zbase32, data) in TEST_DATA {
            let mut with_whitespace = Vec::new();
            for c in zbase32.as_bytes() {
                with_whitespace.extend(&add_whitespace(*c));
            }
            assert_eq!(decode(&with_whitespace, bits).unwrap(), data);
        }
    }

    fn add_whitespace(char: u8) -> Vec<u8> {
        let mut gen = rand::thread_rng();
        let mut result = Vec::new();
        result.extend((0..gen.gen_range(0, 3)).map(|_| *gen.choose(&SPACE_CHARS).unwrap()));
        result.push(char);
        result.extend((0..gen.gen_range(0, 3)).map(|_| *gen.choose(&SPACE_CHARS).unwrap()));
        result
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_decode_full_bytes() {
        let test_data: &[(&[u8], &[u8])] = &[
            (b"6n9hq", &[0xf0, 0xbf, 0xc7, 0x00]),
            (b"4t7ye", &[0xd4, 0x7a, 0x04, 0x00]),
            (b"ybndrfg8ejkmcpqxot1uwisza345h769", &[0x00, 0x44, 0x32, 0x14, 0xc7, 0x42, 0x54, 0xb6,
                                                    0x35, 0xcf, 0x84, 0x65, 0x3a, 0x56, 0xd7, 0xc6,
                                                    0x75, 0xbe, 0x77, 0xdf])
        ];

        for &(zbase32, data) in test_data {
            assert_eq!(decode_full_bytes(zbase32).unwrap(), data);
        }
    }

    #[test]
    fn test_decode_invalid_digits() {
        for string in INVALID_TEST_DATA.iter() {
            assert!(decode(string.as_bytes(), string.as_bytes().len() as u64 * 5).is_err());
            assert!(decode_full_bytes(string.as_bytes()).is_err());
        }
    }

    #[test]
    fn test_decode_superfluous_bits() {
        assert_eq!(decode(b"999", 1).unwrap(), &[0x80]);
        assert_eq!(decode(b"4t7yj", 24).unwrap(), &[0xd4, 0x7a, 0x04]);
        assert_eq!(decode(b"4t7ye9", 25).unwrap(), &[0xd4, 0x7a, 0x04, 0x00]);
        assert_eq!(decode(b"gyh3", 18).unwrap(), &[0x30, 0x39, 0x80]);
    }

    #[test]
    #[should_panic(expected = "zbase64 slice too short")]
    fn test_decode_short_slice() {
        decode(b"oyoy", 4 * 5 + 1).unwrap();
    }

    #[test]
    #[should_panic(expected = "zbase64 slice too short")]
    fn test_decode_short_slice_with_whitespace() {
        decode(b"oyoy ", 4 * 5 + 1).unwrap();
    }

    #[test]
    fn test_encode() {
        for &(bits, zbase32, data) in TEST_DATA {
            assert_eq!(encode(data, bits), zbase32);
        }
    }

    #[test]
    fn test_encode_superfluous_bits() {
        assert_eq!(encode(&[0xff, 0xff], 1), "o");
        assert_eq!(encode(&[0xd4, 0x7a, 0x04, 0xff], 24), "4t7ye");
    }

    #[test]
    #[should_panic(expected = "slice too short")]
    fn test_encode_short_slice() {
        encode(b"1234", 4 * 8 + 1);
    }

    #[test]
    fn test_encode_full_bytes() {
        for &(_, zbase32, data) in TEST_DATA.iter().filter(|&&(i, _, _)| i % 8 == 0) {
            assert_eq!(encode_full_bytes(data), zbase32);
        }
    }

    #[test]
    fn test_validate() {
        for &(_, zbase32, _) in TEST_DATA {
            assert!(validate(zbase32.as_bytes()));
            assert!(validate_str(zbase32));
        }
    }

    #[test]
    fn test_validate_invalid() {
        for string in INVALID_TEST_DATA.iter() {
            assert!(!validate(string.as_bytes()));
            assert!(!validate_str(string));
        }
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn decode_one_mib(b: &mut test::Bencher) {
        let data = random_encoded_data(ONE_MIB);
        b.iter(|| decode_full_bytes(&data).unwrap())
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn encode_one_mib(b: &mut test::Bencher) {
        let data = random_data(ONE_MIB);
        b.iter(|| encode_full_bytes(&data))
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn decode_five_bytes(b: &mut test::Bencher) {
        let data = random_encoded_data(5);
        b.iter(|| decode_full_bytes(&data).unwrap())
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn encode_five_bytes(b: &mut test::Bencher) {
        let data = random_data(5);
        b.iter(|| encode_full_bytes(&data))
    }

    #[cfg(feature = "unstable")]
    #[bench]
    fn validate_one_mib(b: &mut test::Bencher) {
        let data = random_encoded_data(ONE_MIB);
        b.iter(|| validate(&data))
    }

    #[cfg(feature = "unstable")]
    fn random_data(bytes: usize) -> Vec<u8> {
        rand::thread_rng().gen_iter().take(bytes).collect()
    }

    #[cfg(feature = "unstable")]
    fn random_encoded_data(bytes: usize) -> Vec<u8> {
        let mut gen = rand::thread_rng();
        (0..bytes * 8 / 5).map(|_| *gen.choose(ALPHABET).unwrap()).collect()
    }
}
