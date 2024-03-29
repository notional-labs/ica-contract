//! Prost extension traits

use cosmwasm_std::{StdResult};

/// Extension trait for prost messages.
// TODO(tarcieri): decide if this trait should really be sealed or if it should be public
pub trait MessageExt: prost::Message {
    /// Serialize this protobuf message as a byte vector.
    fn to_bytes(&self) -> StdResult<Vec<u8>>;
}

impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> StdResult<Vec<u8>> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes).unwrap();
        Ok(bytes)
    }
}
