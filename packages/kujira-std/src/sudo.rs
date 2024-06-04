use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub enum SudoMsg {
    IcaRegisterCallback(IcaRegisterCallbackData),
    IcaTxCallback(IcaTxCallbackData),
    TransferCallback(TransferCallbackData),
    TransferReceipt(TransferReceiptData),
}

#[cw_serde]
pub struct IcaRegisterCallbackData {
    pub connection_id: String,
    pub account_id: String,
    pub callback: Binary,
    pub result: IcaResult,
}

#[cw_serde]
pub struct IcaTxCallbackData {
    pub connection_id: String,
    pub account_id: String,
    pub sequence: u64,
    pub callback: Binary,
    pub result: IcaResult,
}

#[cw_serde]
pub enum IcaResult {
    Success { data: Binary },
    Error { error: String },
    Timeout {},
}

#[cw_serde]
pub struct TransferCallbackData {
    pub port: String,
    pub channel: String,
    pub sequence: u64,
    pub receiver: String,
    pub denom: String,
    pub amount: String,
    pub memo: String,
    pub result: IcaResult,
    pub callback: Binary,
}

#[cw_serde]
pub struct TransferReceiptData {
    pub port: String,
    pub channel: String,
    pub sequence: u64,
    pub sender: String,
    pub denom: String,
    pub amount: String,
    pub memo: String,
}
