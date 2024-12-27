use crate::error::{Error, Result};
use crate::params::*;
use crate::response::{
    Account, AccountNet, AssetIssueList, Block, BlockList, ChainParameters, Contract, NodeInfo,
    NodeList, Transaction, TransactionInfo, WitnessList, TransferEventResponse, TransferEvent, ConstantContractResponse, EstimateEnergyResponse, AccountResource, CreateTransactionResponse, BroadcastHexResponse
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

    // Get the latest block from the solidity node, 
    // which is the latest confirmed block
    pub async fn get_now_block_solidity(&self) -> Result<Block> {
        self.post("/walletsolidity/getnowblock", EmptyBody::default()).await
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
    ) -> Result<ConstantContractResponse> {
        let params = TriggerConstantContractParams {
            owner_address: owner_address.to_string(),
            contract_address: contract_address.to_string(),
            function_selector: function_selector.to_string(),
            parameter: parameter.to_string(),
            visible: true,
        };
        // debug!("params: {}", serde_json::to_string(&params).unwrap());
        let response = self.post("/wallet/triggerconstantcontract", params).await?;
        Ok(response)
    }

    pub async fn estimate_energy(
        &self,
        owner_address: &str,
        contract_address: &str,
        function_selector: &str,
        parameter: &str,
    ) -> Result<EstimateEnergyResponse> {
        let params = EstimateEnergyParams::new(
            owner_address.to_string(),
            contract_address.to_string(),
            function_selector.to_string(),
            parameter.to_string(),
        );
        // This API is closed by default. To open this interface, the two configuration items vm.estimateEnergy and vm.supportConstant must be enabled in the node configuration file at the same time. 
        self.post("/wallet/estimateenergy", params).await
    }

    pub async fn get_account_resource(&self, address: &str) -> Result<AccountResource> {
        let params = GetAccountParams::new_visible(address.to_string());
        self.post("/wallet/getaccountresource", params).await
    }

    // Creates a TRX transfer transaction
    pub async fn create_transaction(
        &self,
        owner_address: &str,
        to_address: &str,
        amount: i64,
    ) -> Result<Transaction> {
        let params = CreateTransactionParams {
            owner_address: owner_address.to_string(),
            to_address: to_address.to_string(),
            amount,
            visible: true,
        };
        self.post("/wallet/createtransaction", params).await
    }

    /// Broadcasts a signed transaction hex string to the TRON network
    /// 
    /// Parameters:
    /// - transaction: The hex string of the signed transaction
    /// 
    /// Returns a BroadcastHexResponse containing the result and transaction details
    pub async fn broadcast_hex(&self, transaction: String) -> Result<BroadcastHexResponse> {
        let params = BroadcastHexParams{transaction};
        self.post("/wallet/broadcasthex", params).await
    }

    /// Freezes TRX balance to obtain resources (bandwidth or energy) and TRON Power in Stake 2.0
    /// 
    /// Parameters:
    /// - owner_address: Account address in hex format
    /// - resource: Resource type ("BANDWIDTH" or "ENERGY")
    /// - frozen_balance: Amount to freeze in SUN (1 TRX = 1,000,000 SUN)
    /// 
    /// Returns a Transaction object containing the unsigned transaction
    pub async fn freeze_balance_v2(
        &self,
        owner_address: &str,
        resource: &str,
        frozen_balance: i64,
    ) -> Result<Transaction> {
        let params = FreezeBalanceV2Params::new(
            owner_address.to_string(),
            resource.to_string(),
            frozen_balance,
        );
        self.post("/wallet/freezebalancev2", params).await
    }

    /// Unstake TRX from Stake 2.0 system
    /// 
    /// Parameters:
    /// - owner_address: Account address in hex format
    /// - resource: Resource type ("BANDWIDTH" or "ENERGY")
    /// - unfreeze_balance: Amount to unfreeze in SUN (1 TRX = 1,000,000 SUN)
    /// 
    /// Returns a Transaction object containing the unsigned transaction
    /// Note: After unstaking, funds will be locked for 14 days before they can be withdrawn
    pub async fn unfreeze_balance_v2(
        &self,
        owner_address: &str,
        resource: &str,
        unfreeze_balance: i64,
    ) -> Result<Transaction> {
        let params = UnfreezeBalanceV2Params::new(
            owner_address.to_string(),
            resource.to_string(),
            unfreeze_balance,
        );
        self.post("/wallet/unfreezebalancev2", params).await
    }

    /// Delegate bandwidth or energy resources to other accounts in Stake2.0
    /// 
    /// Parameters:
    /// - owner_address: Account address in hex format
    /// - receiver_address: Resource receiver address in hex format
    /// - resource: Resource type ("BANDWIDTH" or "ENERGY")
    /// - balance: Amount to delegate in SUN (1 TRX = 1,000,000 SUN)
    /// - lock: Whether to lock the resource delegation
    /// - lock_period: Lock period in blocks (1 block = 3s). Only valid when lock is true
    ///               For 1 day lock period, use 28800 blocks
    /// 
    /// Returns a Transaction object containing the unsigned transaction
    pub async fn delegate_resource(
        &self,
        owner_address: &str,
        receiver_address: &str,
        resource: &str,
        balance: i64,
        lock: bool,
        lock_period: i64,
    ) -> Result<Transaction> {
        let params = DelegateResourceParams::new(
            owner_address.to_string(),
            receiver_address.to_string(),
            resource.to_string(),
            balance,
            lock,
            lock_period,
        );
        self.post("/wallet/delegateresource", params).await
    }

    /// Cancel the delegation of bandwidth or energy resources to other accounts in Stake2.0
    /// 
    /// Parameters:
    /// - owner_address: Account address in hex format
    /// - receiver_address: Resource receiver address in hex format
    /// - resource: Resource type ("BANDWIDTH" or "ENERGY")
    /// - balance: Amount of resource shares to undelegate in SUN (1 TRX = 1,000,000 SUN)
    /// 
    /// Returns a Transaction object containing the unsigned transaction
    pub async fn undelegate_resource(
        &self,
        owner_address: &str,
        receiver_address: &str,
        resource: &str,
        balance: i64,
    ) -> Result<Transaction> {
        let params = UnDelegateResourceParams::new(
            owner_address.to_string(),
            receiver_address.to_string(),
            resource.to_string(),
            balance,
        );
        self.post("/wallet/undelegateresource", params).await
    }

    /// Activate a new account using an already activated account
    /// 
    /// Parameters:
    /// - owner_address: Transaction initiator address (must be already activated)
    /// - account_address: Account address to be activated
    /// 
    /// Returns a Transaction object containing the unsigned transaction
    /// Note: The transaction must be signed and broadcast within 1 minute
    pub async fn activate_account(
        &self,
        owner_address: &str,
        account_address: &str,
    ) -> Result<Transaction> {
        let params = ActivateAccountParams::new(
            owner_address.to_string(),
            account_address.to_string(),
        );
        self.post("/wallet/createaccount", params).await
    }
}
