#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate enr;

use enr::{CombinedKey, Enr};
use std::str::FromStr;

// Fuzz Enr::decode
fuzz_target!(|data: &[u8]| {
    let base64_str = base64::encode(data);
    let mut enr_str = "enr:".to_string();
    enr_str.push_str(&base64_str);
    let _res: Result<Enr<CombinedKey>, _> = Enr::from_str(&enr_str);
});
