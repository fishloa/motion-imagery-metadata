#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|value: &[u8]| {
    for tag in 0..=u32::from(u16::MAX) {
        let _ = motion_imagery_metadata::decode(tag, value);
    }
});
