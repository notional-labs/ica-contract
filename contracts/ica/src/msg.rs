use cosmwasm_std::{
    CosmosMsg,  Storage,
};
use prost_types::Any;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::ica::{
    MsgRegisterAccount, MsgSubmitTx
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
    RegisteredInterchainAccount(RegisterAccountMsg),
    /// This allows us to transfer *exactly one* native token
    SendTx(SubmitTxMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    /// initial allowlist
    pub default_gas_limit: Option<u64>,
    pub admin: String,
}

/// This is the message for register account
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisterAccountMsg {
    pub owner: String,
    pub connection_id : String,
    pub version: String,
}

/// This is the message for register account
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SubmitTxMsg {
    pub owner: String,
    pub type_url : String,
    pub msg_path: String ,
}