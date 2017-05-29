#[macro_use]
extern crate quickcheck;
extern crate rand;
extern crate zbase32;

use quickcheck::{Arbitrary, Gen};
use rand::Rng;

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
    fn encode_too_long(data: Vec<u8>) -> bool {
        let len_bits = (data.len() as u64) * 8;
        let rand_bits = if len_bits > 0 { rand::thread_rng().gen_range(0, len_bits) } else { 0 };
        println!("datalength: {} bits, encoded length: {} bits", len_bits, rand_bits);
        zbase32::encode(&data, rand_bits);
        true
    }
}

quickcheck! {
    fn decode_partial(data: ZBaseEncodedData) -> bool {
        let len_bits = (data.as_slice().len() as u64) * 5;
        let rand_bits = rand::thread_rng().gen_range(0, len_bits);
        println!("data length: {} bits, encoded length: {} bits", len_bits, rand_bits);
        zbase32::decode(&data.as_slice(), rand_bits).is_ok()
    }
}

quickcheck! {
    fn decode(data: ZBaseEncodedData) -> bool {
        zbase32::decode_full_bytes(&data.as_slice()).is_ok()
    }
}

quickcheck! {
    fn validate(data: ZBaseEncodedData) -> bool {
        zbase32::validate(&data.as_slice())
    }
}

quickcheck! {
    fn validate_str(data: ZBaseEncodedData) -> bool {
        let data = String::from_utf8(data.into_bytes()).unwrap();
        zbase32::validate_str(&data)
    }
}

#[derive(Clone, Debug)]
struct ZBaseEncodedData(Vec<u8>);

impl Arbitrary for ZBaseEncodedData {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let len = g.gen_range(1, 256);
        let content = (0..len).map(|_| *g.choose(zbase32::ALPHABET).unwrap()).collect();
        ZBaseEncodedData(content)
    }
}

impl ZBaseEncodedData {
    fn as_slice(&self) -> &[u8] {
        &self.0
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}
