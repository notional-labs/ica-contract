use cosmwasm_std::{
    CosmosMsg,  Storage,
};
#[derive(Clone, PartialEq, Debug)]
pub struct InitMsg {
    /// initial allowlist
    pub default_gas_limit: Option<u64>,
    pub admin: String,
}