#[macro_use]
extern crate quickcheck;
extern crate zbase32;

use quickcheck::{Arbitrary, Gen};

quickcheck! {
    fn test_recode(data: Vec<u8>) -> bool {
        let encoded = zbase32::encode(&data, data.len() as u64 * 8);
        let encoded_bytes = zbase32::encode_bytes(&data);
        assert_eq!(encoded, encoded_bytes);

        let decoded = zbase32::decode(&encoded.as_bytes(), data.len() as u64 * 8).unwrap();
        let decoded_bytes = zbase32::decode_bytes(&encoded.as_bytes()).unwrap();
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
    fn decode(data: ZBaseEncodedData) -> bool {
        zbase32::decode_bytes(&data.as_slice()).is_ok()
    }
}

#[derive(Clone, Debug)]
struct ZBaseEncodedData(Vec<u8>);

impl Arbitrary for ZBaseEncodedData {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let len = g.gen_range(0, 256);
        let content = (0..len).map(|_| *g.choose(zbase32::ALPHABET).unwrap()).collect();
        ZBaseEncodedData(content)
    }
}

impl ZBaseEncodedData {
    fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
