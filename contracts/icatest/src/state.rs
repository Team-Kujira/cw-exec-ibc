use cw_storage_plus::Map;
use kujira::{
    IcaRegisterCallbackData, IcaTxCallbackData, TransferCallbackData, TransferReceiptData,
};

// Initialize the storage for known interchain accounts.
pub const ICA_REGISTER_CALLBACKS: Map<String, IcaRegisterCallbackData> =
    Map::new("ica_register_callbacks");

pub const ICA_TX_CALLBACKS: Map<String, IcaTxCallbackData> = Map::new("ica_tx_callbacks");

pub const ICA_UNDELEGATE_COMPLETION: Map<String, i64> = Map::new("undelegation_completion");

pub const TRANSFER_CALLBACKS: Map<u64, TransferCallbackData> = Map::new("transfer_callbacks");

pub const TRANSFER_RECEIPTS: Map<u64, TransferReceiptData> = Map::new("transfer_receipts");
