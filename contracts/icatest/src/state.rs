use cw_storage_plus::Map;
use kujira::IcaCallbackData;

// Initialize the storage for known interchain accounts.
pub const INTERCHAIN_CALLBACKS: Map<u64, IcaCallbackData> = Map::new("interchain_callbacks");
