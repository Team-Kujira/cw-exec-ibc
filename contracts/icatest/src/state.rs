use cosmwasm_std::Binary;
use cw_storage_plus::Map;
use kujira::{IcaRegisterCallbackData, IcaTxCallbackData};

// Initialize the storage for known interchain accounts.
pub const ICA_REGISTER_CALLBACKS: Map<String, IcaRegisterCallbackData> =
    Map::new("ica_register_callbacks");

pub const ICA_TX_CALLBACKS: Map<String, IcaTxCallbackData> = Map::new("ica_tx_callbacks");
