use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub enum SudoMsg {
    Callback(IcaCallbackData),
}

#[cw_serde]
pub struct IcaCallbackData {
    pub connection_id: String,
    pub account_id: String,
    pub tx_id: u64,
    pub result_code: u64,
    pub result_data: Binary,
}
pub const RESULT_SUCCESS: u64 = 0u64;
pub const RESULT_FAILURE: u64 = 1u64;
pub const RESULT_TIMEOUT: u64 = 2u64;
