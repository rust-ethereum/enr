#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate enr;
extern crate rlp;

use enr::{CombinedKey, Enr};
use rlp::Decodable;

// Fuzz Enr::decode
fuzz_target!(|data: &[u8]| {
    rlp::decode::<Enr<CombinedKey>>(&data);
});
