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
    },
    SendDelegateTx {
        conn_id: String,
        acc_id: String,
        validator: String,
        delegator: String,
        amount: Coin,
    }
}

#[cw_serde]
pub enum QueryMsg {}
