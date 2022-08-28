#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult, CosmosMsg,
};
use crate::msg::{InitMsg, ExecuteMsg, RegisterAccountMsg};
use crate::error::{ContractError};
use crate::state::{ADMIN};
use crate::ica::{MsgRegisterAccount}
// version info for migration info
const CONTRACT_NAME: &str = "intertx";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;
    ADMIN.set(deps.branch(), Some(admin))?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisteredInterchainAccount(msg) => execute_register_ica(),
        ExecuteMsg::SendTx(msg) => {
            execute_send_tx()
        }
    }
}
 pub fn execute_register_ica(    
    deps: DepsMut,
    env: Env,
    msg: RegisterAccountMsg,
) -> Result<Response, ContractError> {
    let msg_registry_any =  MsgRegisterAccount{
        connection_id: msg.connection_id,
        owner: msg.owner,
        version: msg.version,
    }

 }

 pub fn execute_send_tx() -> Result<Response, ContractError>{
}