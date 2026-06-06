#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _: Result<budlum_core::core::block::Block, _> = bincode::deserialize(data);
});
