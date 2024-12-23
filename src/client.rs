use crate::error::{Error, Result};
use crate::params::*;
use crate::response::{
    Account, AccountNet, AssetIssueList, Block, BlockList, ChainParameters, Contract, NodeInfo,
    NodeList, Transaction, TransactionInfo, WitnessList, TransferEventResponse, EventResponse, Error as ResponseError, TransferEvent
};
use reqwest::{Client as HttpClient, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use url::Url;
use std::str::FromStr;
use log::{error, debug};

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    api_key: Option<String>,
    http_client: HttpClient,
}

pub enum Address {
    Base58(String),
    Hex(String),
}
pub struct TxId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Main,
    Shasta,
    Nile,
}

impl FromStr for Network {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_lowercase().as_str() {
            "main" => Self::Main,
            "shasta" => Self::Shasta,
            "nile" => Self::Nile,
            _ => unimplemented!(),
        })
    }
}

async fn decode_response<T>(res: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let data = res.text().await?;
    // dbg!(&data);

    let s: T =
        serde_json::from_str(&data).map_err(|orig_err| match serde_json::from_str(&data) {
            Err(_) => {
                println!("{}", data);
                // dbg!(&data);
                orig_err.into()
            }
            Ok(r) => Error::ServerError(r),
        })?;

    Ok(s)
}

impl Client {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Client {
            base_url: Url::parse(&base_url).expect("could not parse base_url"),
            api_key,
            http_client: HttpClient::new(),
        }
    }

    pub fn for_network(network: Network, api_key: Option<String>) -> Self {
        let base_url = match network {
            Network::Shasta => "https://api.shasta.trongrid.io".to_string(),
            Network::Main => "https://api.trongrid.io".to_string(),
            Network::Nile => "https://api.nile.trongrid.io".to_string(),
            _ => unimplemented!(),
        };
        Self::new(base_url, api_key)
    }

    pub fn for_shasta(api_key: Option<String>) -> Self {
        Self::for_network(Network::Shasta, api_key)
    }

    pub fn for_main(api_key: Option<String>) -> Self {
        Self::for_network(Network::Main, api_key)
    }

    // todo: for_network(shasta) -> Client (uses trongrid.io api url for shasta

    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        let mut req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json");

        if let Some(api_key) = self.api_key.clone() {
            req = req.header("TRON-PRO-API-KEY", api_key);
        }
        Ok(req)
    }

    fn get_url(&self, path: &str) -> Url {
        self.base_url.join(path).expect("could not parse url")
    }

    async fn req<T, U>(&self, path: &str, method: Method, body: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let res = match method {
            Method::GET => {
                self.prep_req(method, self.get_url(path))
                    .await?
                    .send()
                    .await?
            }
            Method::POST => {
                self.prep_req(method, self.get_url(path))
                    .await?
                    .json(&body)
                    .send()
                    .await?
            }
            _ => unimplemented!(),
        };
        decode_response::<T>(res).await
    }

    pub async fn post<T, U>(&self, path: &str, param: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.req(path, Method::POST, param).await
    }

    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.req(path, Method::GET, EmptyBody::default()).await
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        self.get("/wallet/getnodeinfo").await
    }

    pub async fn list_nodes(&self) -> Result<NodeList> {
        self.get("/wallet/listnodes").await
    }

    pub async fn list_witnesses(&self) -> Result<WitnessList> {
        self.get("/walletsolidity/listwitnesses").await
    }

    pub async fn get_chain_parameters(&self) -> Result<ChainParameters> {
        self.get("/wallet/getchainparameters").await
    }

    pub async fn get_block_by_num(&self, num: u64) -> Result<Block> {
        self.post("/wallet/getblockbynum", GetBlockByNumParams::new(num))
            .await
    }

    pub async fn get_block_by_id(&self, id: &str) -> Result<Block> {
        self.post("/wallet/getblockbyid", GetBlockByIdParams::new(id.into()))
            .await
    }

    pub async fn get_now_block(&self) -> Result<Block> {
        self.post("/wallet/getnowblock", EmptyBody::default()).await
    }

    // num is the number of blocks to query (not the block height)
    pub async fn get_block_by_latest_num(&self, num: u64) -> Result<BlockList> {
        self.post("/wallet/getblockbylatestnum", GetBlockByNumParams::new(num))
            .await
    }

    pub async fn get_block_by_limit_next(&self, start_num: u64, end_num: u64) -> Result<BlockList> {
        self.post(
            "/wallet/getblockbylimitnext",
            GetBlockByRangeParams::new(start_num, end_num),
        )
        .await
    }

    // TODO:
    // walletgetblockbylatestnum
    // getblockbylimitnext
    // createtransaction
    // getnowblock
    // listnodes
    // gettransactioninfobyid
    // gettransactionbyid
    // getchainparameters
    // etc...

    // TODO

    pub async fn get_account(&self, address: Address) -> Result<Account> {
        self.post("/walletsolidity/getaccount", GetAccountParams::new(address))
            .await
    }

    pub async fn get_account_net(&self, address: Address) -> Result<AccountNet> {
        self.post("/wallet/getaccountnet", GetAccountParams::new(address))
            .await
    }

    // TODO: retry if tron node returns an empty object `{}`?
    // This happens when querying a TX in a recently mined block.
    pub async fn get_transaction_by_id(&self, tx_id: TxId) -> Result<Transaction> {
        self.post(
            "/wallet/gettransactionbyid",
            GetTransactionParams::new(tx_id),
        )
        .await
    }

    pub async fn get_transaction_info_by_id(&self, tx_id: TxId) -> Result<TransactionInfo> {
        self.post(
            "/wallet/gettransactioninfobyid",
            GetTransactionParams::new(tx_id),
        )
        .await
    }

    pub async fn get_contract(&self, address: Address) -> Result<Contract> {
        self.post("/wallet/getcontract", GetContractParams::new(address))
            .await
    }

    // TRC10
    //  https://api.trongrid.io/walletsolidity/getassetissuelist
    pub async fn get_asset_issue_list(&self) -> Result<AssetIssueList> {
        self.post("/walletsolidity/getassetissuelist", EmptyBody::default())
            .await
    }

    pub async fn get_transaction_info_by_block_num(&self, block_number: u64) -> Result<Vec<TransactionInfo>> {
        self.post("/wallet/gettransactioninfobyblocknum", GetTransactionByBlockNumParams::new(block_number))
            .await
    }

    pub async fn get_contract_events<T: DeserializeOwned>(
        &self,
        contract_address: &str,
        event_name: &str,
        block_number: Option<u64>,
        min_block_timestamp: Option<u64>,
        max_block_timestamp: Option<u64>,
        limit: Option<u32>
    ) -> Result<T> {
        let mut path = format!(
            "/v1/contracts/{}/events?event_name={}&only_confirmed=true&order_by=block_timestamp,asc",
            contract_address, event_name
        );
        if let Some(limit) = limit {
            path = format!("{}&limit={}", path, limit);
        }else{
            path = format!("{}&limit=200", path);
        }   
        if let Some(block_number) = block_number {
            path = format!("{}&block_number={}", path, block_number);
        }
        if let Some(min_block_timestamp) = min_block_timestamp {
            path = format!("{}&min_block_timestamp={}", path, min_block_timestamp);
        }
        if let Some(max_block_timestamp) = max_block_timestamp {
            path = format!("{}&max_block_timestamp={}", path, max_block_timestamp);
        }
        debug!("path: {}", path);
        self.get(&path).await
    }

    pub async fn get_contract_transfer_events(&self, contract_address: &str, block_number: Option<u64>, min_block_timestamp: Option<u64>, max_block_timestamp: Option<u64>, limit: Option<u32>) -> Result<Vec<TransferEvent>> {
        let mut transfer_events = Vec::new();
        let mut events = self.get_contract_events::<TransferEventResponse>(contract_address, "transfer", block_number, min_block_timestamp, max_block_timestamp, limit)
            .await?;
        match events.success {
            true => transfer_events.append(&mut events.data),
            false => log::error!("Failed to get transfer events: {:?}", events),
        }

        loop {
            if events.meta.page_size >= 200 {
                if let Some(links) = events.meta.links {
                    let next_page = links.next;
                    events = self.get(&next_page).await?;
                    match events.success {
                        true => transfer_events.append(&mut events.data),
                        false => log::error!("Failed to get transfer events: {:?}", events),
                    }
                }
            }else{
                break;
            }
        }
              
        Ok(transfer_events)
    }

    pub async fn trigger_constant_contract(
        &self,
        owner_address: &str,
        contract_address: &str,
        function_selector: &str,
        parameter: &str,
    ) -> Result<String> {
        let params = TriggerConstantContractParams {
            owner_address: owner_address.to_string(),
            contract_address: contract_address.to_string(),
            function_selector: function_selector.to_string(),
            parameter: parameter.to_string(),
            visible: true,
        };

        let response = self.post("/wallet/triggerconstantcontract", params).await?;
        Ok(response)
    }
}
