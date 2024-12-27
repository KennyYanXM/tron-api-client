// use chrono::{Duration, Utc};
// use lazy_static::lazy_static;
// use tokio::sync::{Mutex, MutexGuard};

use tron_api_client::{Address, Client, TxId, client::Network};
use log::info;
// mod data;

// use data::*;

fn get_client() -> Client {
    let client = Client::for_shasta(None);
    client
}

fn get_client_main() -> Client {
    let client = Client::for_main(None);
    client
}

#[tokio::test]
async fn get_node_info() {
    let client = get_client();

    let _ = client
        .get_node_info()
        .await
        .expect("Error fetching node info");
    // dbg!(info);
}

#[tokio::test]
async fn node_list() {
    let client = get_client();

    let _ = client.list_nodes().await.expect("Error fetching node list");
    // dbg!(info);
}

#[tokio::test]
async fn list_witnesses() {
    let client = get_client();

    let info = client
        .list_witnesses()
        .await
        .expect("Error fetching node list");
    dbg!(info);
}

#[tokio::test]
async fn get_block_by_num() {
    let client = get_client();

    let info = client
        .get_block_by_num(10)
        .await
        .expect("Error fetching block by num");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_latest_num() {
    let client = get_client();

    let info = client
        .get_block_by_latest_num(3)
        .await
        .expect("Error fetching num latest blocks");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_limit_next() {
    let client = get_client();

    let info = client
        .get_block_by_limit_next(1_000_000, 1_000_003)
        .await
        .expect("Error fetching block by limit next");
    dbg!(info);
}

#[tokio::test]
async fn get_block_by_num_with_transactions() {
    let client = get_client();

    let info = client
        .get_block_by_num(3412121)
        .await
        .expect("Error fetching block by num");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_id() {
    let client = get_client();

    let info = client
        .get_block_by_id("000000000000000a4efe701d7a03ff578104c6c1995ab70e713c30318b266e90")
        .await
        .expect("Error fetching block by id");
}

#[tokio::test]
async fn get_account() {
    let client = get_client();

    let info = client
        .get_account(Address::Hex(
            "41E552F6487585C2B58BC2C9BB4492BC1F17132CD0".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_2() {
    let client = get_client();

    let info = client
        .get_account(Address::Hex(
            "41a8a07f09def5e6a4462df90068c11abf6224e865".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_net() {
    let client = get_client_main();

    let info = client
        .get_account_net(Address::Hex(
            "41E552F6487585C2B58BC2C9BB4492BC1F17132CD0".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_net2() {
    let client = get_client();

    let info = client
        .get_account_net(Address::Hex(
            "41a8a07f09def5e6a4462df90068c11abf6224e865".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_2_base58() {
    let client = get_client();

    let info = client
        .get_account(Address::Base58("TRLpnm6Uz9s2Fcy3Q235k3SiAEBXGJCNq2".into()))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_transaction_by_id() {
    let client = get_client();

    let info = client
        .get_transaction_by_id(TxId(
            "809e9d9aa5381f32f748618e4d592a58542e21fe794f35959ce811f2a58fc969".into(),
        ))
        .await
        .expect("Error fetching tx by id");
}

#[tokio::test]
async fn get_transaction_info_by_id() {
    let client = get_client();

    let info = client
        .get_transaction_info_by_id(TxId(
            "809e9d9aa5381f32f748618e4d592a58542e21fe794f35959ce811f2a58fc969".into(),
        ))
        .await
        .expect("Error fetching tx info by id");
}

#[tokio::test]
async fn get_now_block() {
    let client = get_client();

    let info = client
        .get_now_block()
        .await
        .expect("Error fetching now block");
}

#[tokio::test]
async fn get_chain_parameters() {
    let client = get_client();

    let info = client
        .get_chain_parameters()
        .await
        .expect("Error fetching chain parameters");
}

// contract 417ca2c40d9aa986b6608e07a68ebf33ea5f19a866

#[tokio::test]
async fn get_contract() {
    let client = get_client();

    let info = client
        .get_contract(Address::Hex(
            "417ca2c40d9aa986b6608e07a68ebf33ea5f19a866".into(),
        ))
        .await
        .expect("Error fetching contract");
}

#[tokio::test]
async fn get_asset_issue_list() {
    let client = get_client();

    let info = client
        .get_asset_issue_list()
        .await
        .expect("Error fetching asset issue list");
}

#[tokio::test]
async fn test_get_contract_transfer_events() {
    env_logger::init();
    let client = Client::for_main(None);
    
    // USDT contract address on mainnet
    let contract = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";
    
    // Test with small limit (single page)
    let events = client.get_contract_transfer_events(
        contract,
        None,
        Some(1621490043000),
        Some(1621490093000),  
        Some(20)              // limit
    ).await.unwrap();
    
    assert!(!events.is_empty());
    info!("events: {:?}", events);
    info!("events: {:?}", events.len());
    assert!(events.len() == 20);
    
    // Verify event fields
    let event = &events[0];
    assert!(!event.transaction_id.is_empty());
    assert!(event.block_number > 0);
    assert!(event.block_timestamp > 0);
    assert_eq!(event.contract_address.to_lowercase(), contract.to_lowercase());
    assert_eq!(event.event_name, "Transfer");
    
    // Test pagination (multi-page)
    let events = client.get_contract_transfer_events(
        contract,
        None,
        Some(1621490043000),
        Some(1621490093000),  
        Some(200)             // limit > 200 to trigger pagination
    ).await.unwrap();
    
    info!("events: {}", events.len());
    assert!(!events.is_empty());
    assert!(events.len() >= 200); // Should have fetched more than one page

    // Test with small limit (single page)
    let events = client.get_contract_transfer_events(
        contract,
        Some(67960725),
        None,
        None,
        Some(200)              // limit
    ).await.unwrap();

    info!("events: {}", events.len());
    assert!(!events.is_empty());
    assert!(events.len() <= 200);
}

#[tokio::test]
async fn test_get_contract_transfer_events_counts() {
    env_logger::init();
    let client = get_client_main();

    for block_number in 67963911..67963912 {
        let events = client.get_contract_transfer_events(
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
            Some(block_number),
            None,
            None,
            None
        ).await.unwrap();
        info!("block_number: {}, events: {}", block_number, events.len());

        for event in events {
            info!("event: {:?}, tx: {}", event.result, event.transaction_id);
        }
    }
}

#[tokio::test]
async fn test_trigger_constant_contract() {
    env_logger::init();
    let client = get_client_main();
    
    let owner_address = "TNXoiAJ3dct8Fjg4M9fkLFh9S2v9TXc32G";  // Example address
    let contract_address = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";  // USDT contract
    let function_selector = "transfer(address,uint256)";
    let parameter = "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c0000000000000000000000000000000000000000000000000000000000000001";
    
    // Call balanceOf function
    let result = client.trigger_constant_contract(
        owner_address,
        contract_address,
        function_selector,
        parameter
    ).await.unwrap();
    info!("result: {:?}", result);
    assert!(result.result.result);
}

#[tokio::test]
async fn test_estimate_energy() {
    env_logger::init();
    let client = get_client_main();
    
    // Test contract parameters
    let owner_address = "TJRabPrwbZy45sbavfcjinPJC18kjpRTv8";  // Example address
    let contract_address = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";  // USDT contract
    let function_selector = "transfer(address,uint256)";
    
    // Example parameter for transfer function (you might need to adjust this based on your needs)
    let parameter = "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c0000000000000000000000000000000000000000000000000000000000001000";

    let result = client
        .estimate_energy(
            owner_address,
            contract_address,
            function_selector,
            parameter,
        )
        .await;

    match result {
        Ok(response) => {
            // Check if we got a valid response
            assert!(response.result.result || response.result.code == "CONTRACT_VALIDATE_ERROR", 
                "Expected either success or CONTRACT_VALIDATE_ERROR");
            
            // If successful, energy_required should be greater than 0
            if response.result.result {
                assert!(response.energy_required > 0, 
                    "Energy required should be greater than 0");
            }
            

            info!("Energy estimation result: {:?}", response);
        },
        Err(e) => {
            // The API might return an error if the node doesn't have vm.estimateEnergy enabled
            info!("Error estimating energy (this might be expected if estimateEnergy is disabled): {:?}", e);
            
        }
    }
}

#[tokio::test]
async fn test_create_transaction() {
    env_logger::init();
    let client = get_client_main();
    
    let owner_address = "TZ4UXDV5ZhNW7fb2AMSbgfAEZ7hWsnYS2g";
    let to_address = "TPswDDCAWhJAZGdHPidFg5nEf8TkNToDX1";
    let amount = 1000;

    let result = client
        .create_transaction(owner_address, to_address, amount)
        .await
        .unwrap();

    info!("Create transaction result: {}", serde_json::to_string(&result).unwrap());
    
    // Verify the response structure
    assert!(result.visible.unwrap());
    assert!(!result.tx_id.is_empty());
    assert!(!result.raw_data_hex.is_empty());
    assert_eq!(result.raw_data.contract.len(), 1);
    
    let contract = &result.raw_data.contract[0];
    assert_eq!(contract.type_field, "TransferContract");
    assert_eq!(contract.parameter.value.amount, Some(amount));
    assert_eq!(contract.parameter.value.owner_address, owner_address);
    assert_eq!(contract.parameter.value.to_address, Some(to_address.to_string()));
}

#[tokio::test]
async fn test_broadcast_hex() {
    env_logger::init();
    let client = get_client_main();
    
    // First create a transaction
    let owner_address = "TZ4UXDV5ZhNW7fb2AMSbgfAEZ7hWsnYS2g";
    let to_address = "TPswDDCAWhJAZGdHPidFg5nEf8TkNToDX1";
    let amount = 1000;

    let tx = client
        .create_transaction(owner_address, to_address, amount)
        .await
        .unwrap();

    // Get the raw transaction hex
    let tx_hex = tx.raw_data_hex;
    
    // Broadcast the transaction hex
    // Note: This will fail without proper signing, but tests the API call structure
    let result = client.broadcast_hex(tx_hex).await;
    
    match result {
        Ok(response) => {
            info!("Broadcast result: {}", serde_json::to_string(&response).unwrap());
            
            // Should have a result field
            assert!(!response.result);  // Will be false since transaction is not signed
            
            // Should have error details
            assert!(response.code.is_some());
            assert!(response.message.is_some());
            
            // Transaction might not be present in error case
            // if let Some(tx) = response.transaction {
            //     assert!(!tx..tx_id.is_empty());
            // }
        },
        Err(e) => {
            // API level errors
            info!("API error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_unfreeze_balance_v2() {
    env_logger::init();
    let client = get_client_main();
    
    let owner_address = "TZ4UXDV5ZhNW7fb2AMSbgfAEZ7hWsnYS2g";
    let resource = "BANDWIDTH";  // or "ENERGY"
    let unfreeze_balance = 1_000_000; // 1 TRX in SUN

    let result = client
        .unfreeze_balance_v2(owner_address, resource, unfreeze_balance)
        .await;

    match result {
        Ok(response) => {
            info!("Unfreeze balance result: {:?}", response);
            
            // Verify the response structure
            assert!(!response.tx_id.is_empty());
            assert_eq!(response.raw_data.contract.len(), 1);
            
            let contract = &response.raw_data.contract[0];
            assert_eq!(contract.type_field, "UnfreezeBalanceV2Contract");
            
            let value = &contract.parameter.value;
            assert_eq!(value.owner_address, owner_address);
        },
        Err(e) => {
            // Could fail if account has no frozen balance
            info!("Error unfreezing balance: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_delegate_resource() {
    env_logger::init();
    let client = get_client_main();
    
    let owner_address = "TZ4UXDV5ZhNW7fb2AMSbgfAEZ7hWsnYS2g";
    let receiver_address = "TPswDDCAWhJAZGdHPidFg5nEf8TkNToDX1";
    let resource = "BANDWIDTH";  // or "ENERGY"
    let balance = 1_000_000; // 1 TRX in SUN
    let lock = true;
    let lock_period = 28800; // 1 day in blocks (3s per block)

    let result = client
        .delegate_resource(
            owner_address,
            receiver_address,
            resource,
            balance,
            lock,
            lock_period
        )
        .await;

    match result {
        Ok(response) => {
            info!("Delegate resource result: {:?}", response);
            
            // Verify the response structure
            assert!(!response.tx_id.is_empty());
            assert_eq!(response.raw_data.contract.len(), 1);
            
            let contract = &response.raw_data.contract[0];
            assert_eq!(contract.type_field, "DelegateResourceContract");
            
            let value = &contract.parameter.value;
            assert_eq!(value.owner_address, owner_address);
            assert_eq!(value.receiver_address.as_ref().unwrap(), receiver_address);
        },
        Err(e) => {
            // Could fail if account has insufficient resources
            info!("Error delegating resource: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_undelegate_resource() {
    env_logger::init();
    let client = get_client_main();
    
    let owner_address = "TZ4UXDV5ZhNW7fb2AMSbgfAEZ7hWsnYS2g";
    let receiver_address = "TPswDDCAWhJAZGdHPidFg5nEf8TkNToDX1";
    let resource = "BANDWIDTH";  // or "ENERGY"
    let balance = 1_000_000; // 1 TRX in SUN

    let result = client
        .undelegate_resource(
            owner_address,
            receiver_address,
            resource,
            balance
        )
        .await;

    match result {
        Ok(response) => {
            info!("Undelegate resource result: {:?}", response);
            
            // Verify the response structure
            assert!(!response.tx_id.is_empty());
            assert_eq!(response.raw_data.contract.len(), 1);
            
            let contract = &response.raw_data.contract[0];
            assert_eq!(contract.type_field, "UnDelegateResourceContract");
            
            let value = &contract.parameter.value;
            assert_eq!(value.owner_address, owner_address);
            assert_eq!(value.receiver_address.as_ref().unwrap(), receiver_address);
            assert_eq!(value.resource.as_ref().unwrap(), resource);
            assert_eq!(value.balance.unwrap(), balance);
        },
        Err(e) => {
            // Could fail if account has no delegated resources
            info!("Error undelegating resource: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_activate_account() {
    env_logger::init();
    let client = get_client_main();
    
    let owner_address = "TZ4UXDV5ZhNW7fb2AMSbgfAEZ7hWsnYS2g";  // Already activated account
    let account_address = "TFNaiXxCcew53fCbn8WwbNpXpSXbPpSXVS"; // New account to activate

    let result = client
        .activate_account(owner_address, account_address)
        .await;

    match result {
        Ok(response) => {
            info!("Activate account result: {:?}", response);
            
            // Verify the response structure
            assert!(!response.tx_id.is_empty());
            assert_eq!(response.raw_data.contract.len(), 1);
            
            let contract = &response.raw_data.contract[0];
            assert_eq!(contract.type_field, "AccountCreateContract");
            
            let value = &contract.parameter.value;
            assert_eq!(value.owner_address, owner_address);
            assert_eq!(value.account_address.as_ref().unwrap(), account_address);
        },
        Err(e) => {
            // Could fail if owner account doesn't have enough TRX or account is already activated
            info!("Error activating account: {:?}", e);
        }
    }
}

