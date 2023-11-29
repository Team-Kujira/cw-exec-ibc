use cosmos_sdk_proto::{
    cosmos::staking::v1beta1::{MsgDelegate, MsgDelegateResponse},
    traits::{MessageExt, TypeUrl},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use kujira::{
    CwICAMsg, IcaRegisterCallbackData, IcaTxCallbackData, KujiraMsg, KujiraQuerier, KujiraQuery,
    ProtobufAny, SudoMsg,
};
use std::str;

use crate::{
    error::ContractError,
    state::{ICA_REGISTER_CALLBACKS, ICA_TX_CALLBACKS},
};
use interface::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

const CONTRACT_NAME: &str = "icatest";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn migrate(_deps: DepsMut<KujiraQuery>, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<KujiraQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateAccount {
            conn_id,
            acc_id,
            version,
            callback,
        } => Ok(
            Response::default().add_message(KujiraMsg::CwIca(CwICAMsg::Register {
                connection_id: conn_id,
                account_id: acc_id,
                version: version,
                callback: Binary::from(callback.as_bytes()),
            })),
        ),
        ExecuteMsg::SendDelegateTx {
            conn_id,
            acc_id,
            validator,
            amount,
            callback,
        } => {
            let address = KujiraQuerier::new(&deps.querier).query_interchain_address(
                env.contract.address,
                conn_id.clone(),
                acc_id.clone(),
            )?;
            let msg = MsgDelegate {
                delegator_address: address.address.clone(),
                validator_address: validator,
                amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                    denom: amount.denom,
                    amount: amount.amount.to_string(),
                }),
            };
            let bytes = msg.to_bytes().unwrap();
            let any = ProtobufAny::new(MsgDelegate::TYPE_URL, bytes);
            Ok(Response::default()
                .add_message(KujiraMsg::CwIca(CwICAMsg::Submit {
                    connection_id: conn_id,
                    account_id: acc_id,
                    msgs: vec![any],
                    memo: "Hello from Kujira".to_string(),
                    timeout: 100000000000u64, // 100 seconds
                    callback: Binary::from(callback.as_bytes()),
                }))
                .add_attribute("Interchain Account Address", address.address))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<KujiraQuery>, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Account { conn_id, acc_id } => query_account(deps, env, conn_id, acc_id),
        QueryMsg::IcaRegisterCallback { callback } => {
            query_ica_register_callback(deps, env, callback)
        }
        QueryMsg::IcaRegisterCallbackKeys {} => query_ica_register_callback_keys(deps, env),
        QueryMsg::IcaTxCallback { callback } => query_ica_tx_callback(deps, env, callback),
        QueryMsg::IcaTxCallbackKeys {} => query_ica_tx_callback_keys(deps, env),
    }
}

fn query_account(
    deps: Deps<KujiraQuery>,
    env: Env,
    conn_id: String,
    acc_id: String,
) -> Result<Binary, ContractError> {
    let querier = KujiraQuerier::new(&deps.querier);
    let res = querier.query_interchain_address(env.contract.address, conn_id, acc_id);
    match res {
        Ok(account) => Ok(to_binary(&account)?),
        Err(e) => Err(e.into()),
    }
}

fn query_ica_register_callback_keys(
    deps: Deps<KujiraQuery>,
    _env: Env,
) -> Result<Binary, ContractError> {
    let keys = ICA_REGISTER_CALLBACKS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            if let Ok((key, _)) = item {
                return key;
            }
            return "".to_string();
        })
        .collect::<Vec<String>>();
    return Ok(to_binary(&keys)?);
}

fn query_ica_tx_callback_keys(deps: Deps<KujiraQuery>, _env: Env) -> Result<Binary, ContractError> {
    let keys = ICA_TX_CALLBACKS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            if let Ok((key, _)) = item {
                return key;
            }
            return "".to_string();
        })
        .collect::<Vec<String>>();
    return Ok(to_binary(&keys)?);
}

fn query_ica_register_callback(
    deps: Deps<KujiraQuery>,
    _env: Env,
    callback: String,
) -> Result<Binary, ContractError> {
    let data = ICA_REGISTER_CALLBACKS.load(deps.storage, callback)?;
    return Ok(to_binary(&data)?);
}

fn query_ica_tx_callback(
    deps: Deps<KujiraQuery>,
    _env: Env,
    callback: String,
) -> Result<Binary, ContractError> {
    let data = ICA_TX_CALLBACKS.load(deps.storage, callback)?;
    return Ok(to_binary(&data)?);
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response> {
    match msg {
        SudoMsg::IcaRegisterCallback(data) => sudo_ica_register_callback(deps, env, data),
        SudoMsg::IcaTxCallback(data) => sudo_ica_tx_callback(deps, env, data),
    }
}

fn sudo_ica_register_callback(
    deps: DepsMut,
    _env: Env,
    data: IcaRegisterCallbackData,
) -> StdResult<Response> {
    // Update the storage record associated with the ica callback.
    ICA_REGISTER_CALLBACKS.save(
        deps.storage,
        str::from_utf8(data.callback.as_slice())?.to_string(),
        &data,
    )?;
    return Ok(Response::default());
}

fn sudo_ica_tx_callback(deps: DepsMut, _env: Env, data: IcaTxCallbackData) -> StdResult<Response> {
    // Update the storage record associated with the ica callback.
    ICA_TX_CALLBACKS.save(
        deps.storage,
        str::from_utf8(data.callback.as_slice())?.to_string(),
        &data,
    )?;
    return Ok(Response::default());
}
