use serde::{Deserialize, de::{DeserializeOwned, Visitor, Error}, Deserializer};
use log::debug;


#[derive(Debug)]
pub struct EventResponse<T: DeserializeOwned> {
    pub success: bool,
    pub data: Vec<T>,
    pub meta: EventMeta,
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for EventResponse<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Helper<T> {
            success: bool,
            data: Vec<T>,
            meta: EventMeta,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(EventResponse {
            success: helper.success,
            data: helper.data,
            meta: helper.meta,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct EventMeta {
    pub at: u64,
    pub page_size: u32,
    pub fingerprint: Option<String>,
    pub links: Option<EventLinks>,
}

#[derive(Debug, Deserialize)]
pub struct EventLinks {
    pub next: String,
}

// Keep the original TransferEventResponse as a type alias for backward compatibility
pub type TransferEventResponse = EventResponse<TransferEvent>;

#[derive(Debug, Deserialize)]
pub struct TransferEvent {
    pub block_number: u64,
    pub block_timestamp: i64,
    pub caller_contract_address: Option<String>,
    pub contract_address: String,
    pub event_index: u32,
    pub event_name: String,
    pub result: TransferEventResult,
    pub transaction_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferEventResult {
    // #[serde(deserialize_with = "deserialize_hex_to_base58")]
    pub from: String,
    // #[serde(deserialize_with = "deserialize_hex_to_base58")]
    pub to: String,
    #[serde(deserialize_with = "deserialize_dec_str_to_u64")]
    pub value: u64,
}



fn deserialize_dec_str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let hex: String = Deserialize::deserialize(deserializer)?;
    Ok(u64::from_str_radix(&hex, 10).map_err(D::Error::custom)?)
}

