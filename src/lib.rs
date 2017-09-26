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

#[cfg_attr(rustfmt, rustfmt_skip)]
const CONVERSION_TABLE: &[u8; 256] = &[
    /*   0 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*   5 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  10 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  15 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  20 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  25 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  30 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  35 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  40 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  45 */ 0xff, 0xff, 0xff, 0xff, 0x12,
    /*  50 */ 0xff, 0x19, 0x1a, 0x1b, 0x1e,
    /*  55 */ 0x1d, 0x07, 0x1f, 0xff, 0xff,
    /*  60 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  65 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  70 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  75 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  80 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  85 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  90 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /*  95 */ 0xff, 0xff, 0x18, 0x01, 0x0c,
    /* 100 */ 0x03, 0x08, 0x05, 0x06, 0x1c,
    /* 105 */ 0x15, 0x09, 0x0a, 0xff, 0x0b,
    /* 110 */ 0x02, 0x10, 0x0d, 0x0e, 0x04,
    /* 115 */ 0x16, 0x11, 0x13, 0xff, 0x14,
    /* 120 */ 0x0f, 0x00, 0x17, 0xff, 0xff,
    /* 125 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 130 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 135 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 140 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 145 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 150 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 155 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 160 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 165 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 170 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 175 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 180 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 185 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 190 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 195 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 200 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 205 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 210 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 215 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 220 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 225 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 230 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 235 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 240 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 245 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 250 */ 0xff, 0xff, 0xff, 0xff, 0xff,
    /* 255 */ 0xff
];

#[inline]
fn value_of_digit(digit: u8) -> u8 {
    CONVERSION_TABLE[digit as usize]
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
pub fn decode(zbase32: &[u8], bits: u64) -> Result<Vec<u8>, &'static str> {
    assert!(zbase32.len() as u64 * 5 >= bits, "zbase32 slice too short");
    let capacity = if bits % 8 == 0 {
        bits / 8
    } else {
        bits / 8 + 1
    } as usize;
    let mut result = Vec::with_capacity(capacity);

    let mut bits_remaining = bits;
    let mut buffer_size: u8 = 0;
    let mut buffer: u16 = !0;
    for digit in zbase32 {
        let value = match value_of_digit(*digit) {
            0xff => return Err("not a zbase32 digit"),
            v => v
        };
        buffer = (buffer << 5) | u16::from(value);
        buffer_size += 5;
        if bits_remaining < 8 && u64::from(buffer_size) >= bits_remaining {
            break;
        }
        if buffer_size >= 8 {
            let byte = (buffer >> (buffer_size - 8)) as u8;
            result.push(byte);
            bits_remaining -= 8;
            buffer_size -= 8;
        }
    }
    if bits_remaining > 0 {
        let trim_right = buffer_size - bits_remaining as u8;
        buffer >>= trim_right;
        buffer_size -= trim_right;
        let byte = (buffer << (8_u8 - buffer_size)) as u8;
        result.push(byte);
    }
    debug_assert_eq!(capacity, result.len());
    Ok(result)
}

/// Decode given zbase32 encoded string
///
/// Just like `decode` but doesn't allow decoding with bit precision.
///
/// This decodes full bytes. For instance, if you have `b"yy"`, you'll get one
/// byte back. `b"yy"` can enode 10 bits (2 * 5) which is truncated at the next
/// lower byte boundary.
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
    let size = zbase.len() as u64 * 5;
    decode(zbase, size / 8 * 8)
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
    decode(zbase32.as_bytes(), bits)
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
    let mut bit_offset: u8 = 16;
    let mut remaining = data;
    let mut buffer = !0;

    while bits_remaining > 0 {
        if bit_offset >= 8 {
            if let Some((first, others)) = remaining.split_first() {
                buffer = buffer << 8 | u16::from(*first);
                remaining = others;
                bit_offset -= 8;
            }
        }

        let unused_bits = 5_u64.saturating_sub(bits_remaining);
        let index = (buffer >> (unused_bits as u8 + 16 - 5 - bit_offset) << unused_bits) & 0x1f;
        result.push(ALPHABET[index as usize]);

        bit_offset += 5;
        bits_remaining -= 5 - unused_bits;
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
    data.iter().all(|i| value_of_digit(*i) != 0xff)
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
    #[cfg(feature = "unstable")]
    use tests::rand::Rng;

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

    const INVALID_TEST_DATA: &[&str] = &["ybndrfg8ejkmcpqxot1uwisza345H769", "bnℕe", "uv", "l"];

    #[cfg(feature = "unstable")]
    const ONE_MIB: usize = 1048576;

    #[test]
    fn test_decode() {
        for &(bits, zbase32, data) in TEST_DATA {
            assert_eq!(decode(zbase32.as_bytes(), bits).unwrap(), data);
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_decode_full_bytes() {
        let test_data: &[(&[u8], &[u8])] = &[
            (b"9",     &[]),
            (b"y9",    &[0x07]),
            (b"6n9hq", &[0xf0, 0xbf, 0xc7]),
            (b"4t7ye", &[0xd4, 0x7a, 0x04]),
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
    #[should_panic(expected = "zbase32 slice too short")]
    fn test_decode_short_slice() {
        decode(b"oyoy", 4 * 5 + 1).unwrap();
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

    #[test]
    fn test_valid_and_invalid_chars() {
        for char in (0_u16..256).map(|i| i as u8) {
            let bytes = &[char];
            if ALPHABET.contains(&char) {
                assert_eq!(ALPHABET[value_of_digit(char) as usize], char);
                assert_eq!(encode(&decode(bytes, 5).unwrap(), 5).as_bytes(), bytes);
                assert!(validate(bytes));
            } else {
                assert!(value_of_digit(char) == 0xff);
                assert!(decode_full_bytes(bytes).is_err());
                assert!(!validate(bytes));
            }
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
        (0..bytes * 8 / 5)
            .map(|_| *gen.choose(ALPHABET).unwrap())
            .collect()
    }
}
