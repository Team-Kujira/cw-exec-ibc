use cosmos_sdk_proto::cosmos::base::abci::v1beta1::{MsgData, TxMsgData};
use cosmos_sdk_proto::traits::Message;
use cosmos_sdk_proto::Any;
use cosmos_sdk_proto::{
    cosmos::staking::v1beta1::{
        MsgDelegate, MsgDelegateResponse, MsgUndelegate, MsgUndelegateResponse,
    },
    traits::MessageExt,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Storage,
};
use cw2::set_contract_version;
use kujira::{
    CwICAMsg, IcaRegisterCallbackData, IcaTxCallbackData, KujiraMsg, KujiraQuerier, KujiraQuery,
    ProtobufAny, SudoMsg,
};
use std::str;

use crate::state::ICA_UNDELEGATE_COMPLETION;
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
            let any = ProtobufAny::new("/cosmos.staking.v1beta1.MsgDelegate", bytes);
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
        ExecuteMsg::SendUndelegateTx {
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
            let msg = MsgUndelegate {
                delegator_address: address.address.clone(),
                validator_address: validator,
                amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                    denom: amount.denom,
                    amount: amount.amount.to_string(),
                }),
            };

            let default = -1i64;
            ICA_UNDELEGATE_COMPLETION.save(deps.storage, callback.to_owned(), &default)?;
            let bytes = msg.to_bytes().unwrap();
            let any = ProtobufAny::new("/cosmos.staking.v1beta1.MsgUndelegate", bytes);
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
        QueryMsg::IcaUndelegateCompletion { callback } => {
            query_ica_undelegation_completion(deps, env, callback)
        }
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

fn query_ica_undelegation_completion(
    deps: Deps<KujiraQuery>,
    _env: Env,
    callback: String,
) -> Result<Binary, ContractError> {
    let data = ICA_UNDELEGATE_COMPLETION.load(deps.storage, callback)?;
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

// before sdk v0.46
fn parse_old_callback_data(storage: &mut dyn Storage, callback: String, resp_arr: Vec<MsgData>) {
    let mut timestamp: i64 = 0;
    for data in resp_arr {
        let decoded_result = MsgUndelegateResponse::decode(&data.data[..]);
        if let Ok(decoded) = decoded_result {
            if let Some(completion) = decoded.completion_time {
                timestamp = completion.seconds;
            }
        }
    }
    let _ = ICA_UNDELEGATE_COMPLETION.save(storage, callback.to_owned(), &timestamp);
}

// after sdk v0.46
fn parse_callback_data(storage: &mut dyn Storage, callback: String, resp_arr: Vec<Any>) {
    let mut timestamp: i64 = 0;
    for data in resp_arr {
        let decoded_result = MsgUndelegateResponse::decode(&data.value[..]);
        if let Ok(decoded) = decoded_result {
            if let Some(completion) = decoded.completion_time {
                timestamp = completion.seconds;
            }
        }
    }
    let _ = ICA_UNDELEGATE_COMPLETION.save(storage, callback.to_owned(), &timestamp);
}

fn sudo_ica_tx_callback(deps: DepsMut, _env: Env, data: IcaTxCallbackData) -> StdResult<Response> {
    // Update the storage record associated with the ica callback.
    let callbackkey = str::from_utf8(data.callback.as_slice())?.to_string();
    ICA_TX_CALLBACKS.save(deps.storage, callbackkey.to_owned(), &data)?;

    if let Ok(_) = ICA_UNDELEGATE_COMPLETION.load(deps.storage, callbackkey.to_owned()) {
        match data.result {
            kujira::IcaResult::Success { data } => {
                let tx_msg_data_result = TxMsgData::decode(&data[..]);
                if let Ok(tx_msg_data) = tx_msg_data_result {
                    // try parsing old format
                    parse_old_callback_data(deps.storage, callbackkey.to_owned(), tx_msg_data.data);
                    // try parsing latest format
                    parse_callback_data(
                        deps.storage,
                        callbackkey.to_owned(),
                        tx_msg_data.msg_responses,
                    );
                }
            }
            _ => {}
        }
    }

    return Ok(Response::default());
}
