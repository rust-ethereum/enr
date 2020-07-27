#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate enr;

use enr::{c_secp256k1::SecretKey, Enr};
use std::str::FromStr;

// Fuzz Enr::decode
fuzz_target!(|data: &[u8]| {
    let utf8_str = String::from_utf8_lossy(data);
    let _: Result<Enr<SecretKey>, _> = Enr::from_str(&utf8_str);
});
