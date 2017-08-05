#![cfg(feature = "unstable")]
#![cfg_attr(test, feature(test))]

const ONE_MIB: usize = 1048576;

extern crate zbase32;
extern crate rand;
extern crate test;

use rand::Rng;

#[bench]
fn decode_one_mib(b: &mut test::Bencher) {
    let data = random_encoded_data(ONE_MIB);
    b.iter(|| zbase32::decode_full_bytes(&data).unwrap())
}

#[bench]
fn encode_one_mib(b: &mut test::Bencher) {
    let data = random_data(ONE_MIB);
    b.iter(|| zbase32::encode_full_bytes(&data))
}

#[bench]
fn decode_five_bytes(b: &mut test::Bencher) {
    let data = random_encoded_data(5);
    b.iter(|| zbase32::decode_full_bytes(&data).unwrap())
}

#[bench]
fn encode_five_bytes(b: &mut test::Bencher) {
    let data = random_data(5);
    b.iter(|| zbase32::encode_full_bytes(&data))
}

#[bench]
fn validate_one_mib(b: &mut test::Bencher) {
    let data = random_encoded_data(ONE_MIB);
    b.iter(|| zbase32::validate(&data))
}

fn random_data(bytes: usize) -> Vec<u8> {
    rand::thread_rng().gen_iter().take(bytes).collect()
}

fn random_encoded_data(bytes: usize) -> Vec<u8> {
    let mut gen = rand::thread_rng();
    (0..bytes * 8 / 5).map(|_| *gen.choose(zbase32::ALPHABET).unwrap()).collect()
}
