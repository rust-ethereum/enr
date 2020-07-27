#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate enr;
extern crate rlp;

use enr::{ed25519_dalek::Keypair, Enr};
use rlp::Decodable;

// Fuzz Enr::decode
fuzz_target!(|data: &[u8]| {
    rlp::decode::<Enr<Keypair>>(&data);
});
