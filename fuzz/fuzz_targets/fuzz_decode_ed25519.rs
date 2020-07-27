#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate enr;
extern crate rlp;

use enr::{Enr, ed25519_dalek::Keypair};
use rlp::Decodable;

// Fuzz Enr::decode
fuzz_target!(|data: &[u8]| {
    rlp::decode::<Enr<Keypair>>(&data);
});
