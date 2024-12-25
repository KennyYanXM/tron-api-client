use crate::client::{Address, TxId};
use serde::Serializer;
use serde_derive::Serialize;

/// Parameters used to get series images with
/// [`Client::series_images_query`](../client/struct.Client.html#method.series_images_query).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockByNumParams {
    num: u64,
}

impl GetBlockByNumParams {
    pub fn new(num: u64) -> GetBlockByNumParams {
        GetBlockByNumParams { num }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockByRangeParams {
    start_num: u64,
    end_num: u64,
}

impl GetBlockByRangeParams {
    pub fn new(start_num: u64, end_num: u64) -> GetBlockByRangeParams {
        GetBlockByRangeParams { start_num, end_num }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockByIdParams {
    #[serde(rename = "value")]
    id: String,
}

impl GetBlockByIdParams {
    pub fn new(id: String) -> GetBlockByIdParams {
        GetBlockByIdParams { id }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountParams {
    #[serde(rename = "address")]
    address: String,
    // true if address is in base58...
    visible: bool,
}

impl GetAccountParams {
    pub fn new(address: Address) -> GetAccountParams {
        let (address, visible) = match address {
            Address::Base58(addr) => (addr, true),
            Address::Hex(addr) => (addr, false),
        };

        GetAccountParams { address, visible }
    }

    pub fn new_visible(address: String) -> GetAccountParams {
        GetAccountParams {
            address,
            visible: true,
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionParams {
    #[serde(rename = "value")]
    id: String,
}

impl GetTransactionParams {
    pub fn new(tx_id: TxId) -> GetTransactionParams {
        GetTransactionParams { id: tx_id.0 }
    }
}

#[derive(Debug, Default)]
pub struct EmptyBody {}
impl serde::Serialize for EmptyBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("")
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetContractParams {
    #[serde(rename = "value")]
    address: String,
}

impl GetContractParams {
    pub fn new(address: Address) -> GetContractParams {
        let address = match address {
            Address::Base58(addr) => addr,
            Address::Hex(addr) => addr,
        };

        GetContractParams { address }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionByBlockNumParams {
    num: u64,
}

impl GetTransactionByBlockNumParams {
    pub fn new(num: u64) -> GetTransactionByBlockNumParams {
        GetTransactionByBlockNumParams { num }
    }
}

#[derive(Debug, Serialize)]
pub struct GetContractEventsParams {
    pub contract_address: String,
    pub event_name: String,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TriggerConstantContractParams {
    pub owner_address: String,
    pub contract_address: String,
    pub function_selector: String,
    pub parameter: String,
    pub visible: bool,
}

#[derive(Debug, Serialize)]
pub struct EstimateEnergyParams {
    pub owner_address: String,
    pub contract_address: String,
    pub function_selector: String,
    pub parameter: String,
    pub visible: bool,
}

impl EstimateEnergyParams {
    pub fn new(
        owner_address: String,
        contract_address: String,
        function_selector: String,
        parameter: String,
    ) -> Self {
        Self {
            owner_address,
            contract_address,
            function_selector,
            parameter,
            visible: true,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateTransactionParams {
    pub owner_address: String,
    pub to_address: String,
    pub amount: i64,
    pub visible: bool,
}

impl CreateTransactionParams {
    pub fn new(owner_address: String, to_address: String, amount: i64) -> Self {
        Self {
            owner_address,
            to_address,
            amount,
            visible: true,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BroadcastHexParams {
    pub transaction: String
}

