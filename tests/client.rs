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

    let info = client
        .get_node_info()
        .await
        .expect("Error fetching node info");
    // dbg!(info);
}

#[tokio::test]
async fn node_list() {
    let client = get_client();

    let info = client.list_nodes().await.expect("Error fetching node list");
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
    let client = get_client_main();
    
    // USDT contract
    let contract = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";
    
    // Call balanceOf function
    let result = client.trigger_constant_contract(
        contract,
        "balanceOf(address)", 
        "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c",
        None
    ).await.unwrap();
    
    assert!(!result.is_empty());
}

