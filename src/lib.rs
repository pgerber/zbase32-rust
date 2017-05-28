//! Encoding and decoding of zbase32.
//!
//! This is an implementation of the human-oriented base-32 encoding called
//! [zbase32](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt).

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

const ALPHABET: &[u8; 32] = b"ybndrfg8ejkmcpqxot1uwisza345h769";

fn value_of_digit(digit: u8) -> Result<u8, &'static str> {
    match digit {
        b'y' => Ok(0x00),
        b'b' => Ok(0x01),
        b'n' => Ok(0x02),
        b'd' => Ok(0x03),
        b'r' => Ok(0x04),
        b'f' => Ok(0x05),
        b'g' => Ok(0x06),
        b'8' => Ok(0x07),
        b'e' => Ok(0x08),
        b'j' => Ok(0x09),
        b'k' => Ok(0x0a),
        b'm' => Ok(0x0b),
        b'c' => Ok(0x0c),
        b'p' => Ok(0x0d),
        b'q' => Ok(0x0e),
        b'x' => Ok(0x0f),
        b'o' => Ok(0x10),
        b't' => Ok(0x11),
        b'1' => Ok(0x12),
        b'u' => Ok(0x13),
        b'w' => Ok(0x14),
        b'i' => Ok(0x15),
        b's' => Ok(0x16),
        b'z' => Ok(0x17),
        b'a' => Ok(0x18),
        b'3' => Ok(0x19),
        b'4' => Ok(0x1a),
        b'5' => Ok(0x1b),
        b'h' => Ok(0x1c),
        b'7' => Ok(0x1d),
        b'6' => Ok(0x1e),
        b'9' => Ok(0x1f),
        _ => Err("not a zbase32 digit"),
    }
}

/// Encode first N `bits` with zbase32.
///
/// # Panics
///
/// Panics if `data` is shorter than N `bits`.
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
    let mut remaining = if data.len() > 2 { &data[2..] } else { &[] };
    let mut bit_offset = 0;
    let mut buffer = (*data.get(0).unwrap_or(&0) as u16) << 8 | *data.get(1).unwrap_or(&0) as u16;
    while bits_remaining > 0 {
        let index = {
            let mask = if bits_remaining >= 5 {
                0x1f
            } else {
                // trim superfluous bits at end of `data`
                0x1f >> (5 - bits_remaining) << (5 - bits_remaining)
            };
            (buffer >> (16 - 5 - bit_offset)) & mask
        };
        result.push(ALPHABET[index as usize]);

        bit_offset += 5;
        bits_remaining = bits_remaining.saturating_sub(5);

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
/// ```
/// use zbase32;
///
/// let data = "Just an arbitrary sentence.";
/// assert_eq!(zbase32::encode_bytes(data.as_bytes()),
///            "jj4zg7bycfznyam1cjwzehubqjh1yh5fp34gk5udcwzy");
/// ```
#[inline]
pub fn encode_bytes(data: &[u8]) -> String {
    encode(data, data.len() as u64 * 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    const TEST_DATA: &[(u64, &str, &[u8])] = &[
        (0,   "",       &[]),
        (1,   "y",      &[0x00]),
        (1,   "o",      &[0x80]),
        (1,   "o",      &[0xff, 0xff, 0xff]),
        (2,   "e",      &[0x40]),
        (2,   "a",      &[0xc0]),
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

    #[test]
    fn test_encode() {
        for &(bits, zbase32, data) in TEST_DATA {
            assert_eq!(encode(data, bits), zbase32);
        }
    }

    #[test]
    #[should_panic(expected = "slice too short")]
    fn test_encode_short_slice() {
        encode(b"1234", 4 * 8 + 1);
    }

    #[test]
    fn test_encode_bytes() {
        for &(_, zbase32, data) in TEST_DATA.iter().filter(|&&(i, _, _)| i % 8 == 0) {
            assert_eq!(encode_bytes(data), zbase32);
        }
    }
}
