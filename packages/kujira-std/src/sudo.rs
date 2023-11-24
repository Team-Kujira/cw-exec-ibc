use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub enum SudoMsg {
    IcaCallback(IcaCallbackData),
}

#[cw_serde]
pub struct IcaCallbackData {
    pub connection_id: String,
    pub account_id: String,
    pub tx_id: u64,
    pub result: IcaResult,
}

#[cw_serde]
pub enum IcaResult {
    Success { data: Binary },
    Error { error: String },
    Timeout {},
}
