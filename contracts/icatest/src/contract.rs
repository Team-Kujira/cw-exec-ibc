use cosmos_sdk_proto::{
    cosmos::staking::v1beta1::MsgDelegate,
    traits::{MessageExt, TypeUrl},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use kujira::{InterTxMsg, KujiraMsg, KujiraQuery, ProtobufAny};

use crate::error::ContractError;
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
    _deps: DepsMut<KujiraQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateAccount {
            conn_id,
            acc_id,
            version,
        } => Ok(
            Response::default().add_message(KujiraMsg::Intertx(InterTxMsg::Register {
                connection_id: conn_id,
                account_id: acc_id,
                version,
            })),
        ),
        ExecuteMsg::SendDelegateTx {
            conn_id,
            acc_id,
            validator,
            delegator,
            amount,
        } => {
            let msg = MsgDelegate {
                delegator_address: delegator,
                validator_address: validator,
                amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
                    denom: amount.denom,
                    amount: amount.amount.to_string(),
                }),
            };
            let bytes = msg.to_bytes().unwrap();
            let any = ProtobufAny::new(MsgDelegate::TYPE_URL.to_string(), bytes.into());
            // let type_url = MsgDelegate::TYPE_URL;
            // let msg = cosmwasm_std::to_vec(&msg).unwrap();
            // let msg = ProtobufAny {
            //     type_url: type_url.to_string(),
            //     value: msg,
            // };
            Ok(
                Response::default().add_message(KujiraMsg::Intertx(InterTxMsg::Submit {
                    connection_id: conn_id,
                    account_id: acc_id,
                    msgs: vec![any],
                    memo: "Hello from Kujira".to_string(),
                    timeout: 100000000000u64,
                })),
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<KujiraQuery>, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    unimplemented!()
}
