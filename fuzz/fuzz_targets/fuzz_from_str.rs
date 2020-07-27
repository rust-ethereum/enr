#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate enr;
extern crate rlp;

use enr::{Enr, CombinedKey};
use rlp::Decodable;
use std::str::FromStr;

// Fuzz Enr::decode
fuzz_target!(|data: &[u8]| {
    let utf8_str = String::from_utf8_lossy(data);
    let _: Result<Enr<CombinedKey>, _> = Enr::from_str(&utf8_str);
});
