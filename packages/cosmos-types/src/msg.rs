//! Transaction messages

use crate::{prost_ext::MessageExt, proto, Any};
use cosmwasm_std::{StdResult,StdError};
use core::convert::TryFrom;
use core::convert::TryInto;

/// Types which impl this trait map one-to-one with a corresponding Protocol
/// Buffers type, but can assert additional invariants and/or additional
/// functionality beyond the raw proto, as well as providing a more idiomatic
/// Rust type to work with.
pub trait Msg:
    Clone + Sized + TryFrom<Self::Proto, Error = StdError> + Into<Self::Proto>
{
    /// Protocol Buffers type
    type Proto: MsgProto;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> StdResult<Self> {
        Self::Proto::from_any(any)?.try_into()
    }    

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> StdResult<Any> {
        self.clone().into_any()
    }

    /// Convert this message proto into [`Any`].
    fn into_any(self) -> StdResult<Any> {
        self.into().to_any()
    }
}

/// Proto types which can be used as a [`Msg`].
pub trait MsgProto: Default + MessageExt + Sized {
    /// Type URL value
    const TYPE_URL: &'static str;

    /// Parse this message proto from [`Any`].
    fn from_any(any: &Any) -> StdResult<Self> {
        if any.type_url == Self::TYPE_URL {
            Ok(Self::decode(&*any.value).unwrap())
        } else {
            return Err(StdError::generic_err("can't unmarshal from any"));
        }
    }    

    /// Serialize this message proto as [`Any`].
    fn to_any(&self) -> StdResult<Any> {
        self.to_bytes().map(|bytes| Any {
            type_url: Self::TYPE_URL.to_owned(),
            value: bytes,
        })
    }
}
