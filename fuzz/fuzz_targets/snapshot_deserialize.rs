#![no_main]

use budlum_core::chain::snapshot::{StateSnapshot, StateSnapshotV2};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = StateSnapshot::from_bytes(data);
    let _ = StateSnapshotV2::from_bytes(data);
});
