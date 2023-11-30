use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAccount {
        conn_id: String,
        acc_id: String,
        version: String,
        callback: String,
    },
    SendDelegateTx {
        conn_id: String,
        acc_id: String,
        validator: String,
        amount: Coin,
        callback: String,
    },
    SendUndelegateTx {
        conn_id: String,
        acc_id: String,
        validator: String,
        amount: Coin,
        callback: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Account { conn_id: String, acc_id: String },
    IcaRegisterCallbackKeys {},
    IcaRegisterCallback { callback: String },
    IcaTxCallbackKeys {},
    IcaTxCallback { callback: String },
    IcaUndelegateCompletion { callback: String },
}
