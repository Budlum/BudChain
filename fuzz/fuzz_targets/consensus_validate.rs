#![no_main]

use arbitrary::Arbitrary;
use budlum_core::core::block::BlockHeader;
use budlum_core::core::address::Address;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzBlockHeader {
    index: u64,
    hash: [u8; 32],
    previous_hash: [u8; 32],
    timestamp: u128,
    producer: [u8; 32],
    has_producer: bool,
    state_root: [u8; 32],
    tx_root: [u8; 32],
}

fuzz_target!(|data: FuzzBlockHeader| {
    let producer = if data.has_producer {
        Some(Address::from(data.producer))
    } else {
        None
    };

    let header = BlockHeader {
        index: data.index,
        hash: hex::encode(data.hash),
        previous_hash: hex::encode(data.previous_hash),
        timestamp: data.timestamp,
        producer,
        state_root: hex::encode(data.state_root),
        tx_root: hex::encode(data.tx_root),
    };

    let hex_hash = header.hash.clone();
    assert!(!hex_hash.is_empty());

    let _serialized = bincode::serialize(&header);
});
