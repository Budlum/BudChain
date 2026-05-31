use crate::{Block, BlockHeader, Transaction};
use crate::consensus::pos::SlashingEvidence;
use crate::network::protocol::NetworkMessage;

#[allow(clippy::all)]
pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/budlum.network.rs"));
}
