#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _: Result<budlum_core::core::transaction::Transaction, _> = bincode::deserialize(data);
});
