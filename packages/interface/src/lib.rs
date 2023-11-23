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
        tx_id: u64,
    },
    SendDelegateTx {
        conn_id: String,
        acc_id: String,
        validator: String,
        amount: Coin,
        tx_id: u64,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Account { conn_id: String, acc_id: String },
    IcaCallback { tx_id: u64 },
}
