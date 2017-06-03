#[macro_use]
extern crate quickcheck;
extern crate rand;
extern crate zbase32;

use quickcheck::{Arbitrary, Gen, TestResult};
use rand::Rng;

const ALPHABET_WITH_WHITESPACE: &[u8; 35] = b"ybndrfg8ejkmcpqxot1uwisza345h769 \t\n";
const WHITESPACE: &[u8; 3] = b" \t\n";

quickcheck! {
    fn test_recode(data: Vec<u8>) -> bool {
        let encoded = zbase32::encode(&data, data.len() as u64 * 8);
        let encoded_bytes = zbase32::encode_full_bytes(&data);
        assert_eq!(encoded, encoded_bytes);

        let decoded = zbase32::decode(&encoded.as_bytes(), data.len() as u64 * 8).unwrap();
        let decoded_bytes = zbase32::decode_full_bytes(&encoded.as_bytes()).unwrap();
        assert_eq!(decoded[..], decoded_bytes[..data.len()]);

        // `decoded_bytes` may add an additional byte since it doesn't know the original size.
        // The size is simply assumed to be `encoded.len() * 5`.
        if (encoded.len() * 5) % 8 == 0 {
            assert_eq!(decoded_bytes.len(), data.len())
        } else {
            assert_eq!(decoded_bytes[data.len()..], [0x00][..]);
        }

        data == decoded
    }
}

quickcheck! {
    fn try_decode_ok(data: Vec<u8>) -> TestResult {
        if zbase32::validate(&data) {
            TestResult::from_bool(zbase32::decode_full_bytes(&data).is_ok())
        } else {
            TestResult::discard()
        }
    }
}

quickcheck! {
    fn try_decode_err(data: Vec<u8>) -> TestResult {
        if zbase32::validate(&data) {
            TestResult::discard()
        } else {
            TestResult::from_bool(zbase32::decode_full_bytes(&data).is_err())
        }
    }
}

quickcheck! {
    fn data_len_exceeds_bits_when_encoding(data: Vec<u8>, arbitrary: u8) -> TestResult {
        TestResult::must_fail(move || {
            let len = data.len() as u64 * 8 + 1 + arbitrary as u64;
            zbase32::encode(&data, len);
        })
    }
}

quickcheck! {
    fn data_len_exceeds_bits_when_ecoding(data: ZBaseEncodedData, arbitrary: u8) -> TestResult {
        TestResult::must_fail(move || {
            let len = data.as_bytes().len() as u64 * 5 + 1 + arbitrary as u64;
            let _ = zbase32::decode(data.as_bytes(), len);
        })
    }
}

quickcheck! {
    fn encode_too_long(data: Vec<u8>) -> bool {
        let len_bits = (data.len() as u64) * 8;
        let rand_bits = if len_bits > 0 { rand::thread_rng().gen_range(0, len_bits) } else { 0 };
        println!("data length: {} bits, requested length: {} bits", len_bits, rand_bits);
        zbase32::encode(&data, rand_bits);
        true
    }
}

quickcheck! {
    fn recode_partial(data: ZBaseEncodedData) -> bool {
        let rand_bits = if data.bit_len() > 0 {
            rand::thread_rng().gen_range(0, data.bit_len())
        } else {
            0
        };
        println!("data length: {} bits, requested length: {} bits", data.bit_len(), rand_bits);
        let decoded1 = zbase32::decode(&data.as_bytes(), rand_bits).unwrap();
        let encoded = zbase32::encode(&decoded1, rand_bits);
        cmp_ignore_whitespace(data.as_bytes(), encoded.as_bytes());
        let decoded2 = zbase32::decode_str(&encoded, rand_bits).unwrap();
        decoded1 == decoded2
    }
}

quickcheck! {
    fn decode(data: ZBaseEncodedData) -> bool {
        zbase32::decode_full_bytes(&data.as_bytes()).is_ok()
    }
}

quickcheck! {
    fn validate(data: ZBaseEncodedData) -> bool {
        zbase32::validate(&data.as_bytes())
    }
}

quickcheck! {
    fn validate_str(data: ZBaseEncodedData) -> bool {
        let data = String::from_utf8(data.into_bytes()).unwrap();
        zbase32::validate_str(&data)
    }
}

fn cmp_ignore_whitespace<'a, A, B>(a: A, b: B) -> bool
    where A: IntoIterator<Item = &'a u8>,
          B: IntoIterator<Item = &'a u8>
{
    a.into_iter()
        .filter(|i| !WHITESPACE.contains(i))
        .eq(b.into_iter().filter(|i| !WHITESPACE.contains(i)))
}

#[derive(Clone, Debug)]
struct ZBaseEncodedData(Vec<u8>);

impl Arbitrary for ZBaseEncodedData {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let len = g.gen_range(0, 256);
        let content = (0..len).map(|_| *g.choose(ALPHABET_WITH_WHITESPACE).unwrap()).collect();
        ZBaseEncodedData(content)
    }
}

impl ZBaseEncodedData {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    fn bit_len(&self) -> u64 {
        self.0.iter().filter(|i| !WHITESPACE.contains(i)).count() as u64 * 5
    }
}
