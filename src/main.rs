use hex_literal::hex; 
use web3::{futures::StreamExt, types::{H160, U64, TransactionId, H256}, transports::WebSocket, Web3, contract::{Contract, Options}};
use std::{fs::File, io::Read};

// WebSocket URL to connect to 
const WSS_URL: &str = "wss://...";

// Main function
#[tokio::main]
// Subscribes to new transactions and processes them
async fn main(){

    // Create WebSocket connection
    let transport = WebSocket::new(WSS_URL).await.expect("Transport not connected.");
    // Initialize Web3 instance
    let web3 = Web3::new(transport);   

    // V2 router contract address (without 0x prefix)
    let router_address: H160 = "a5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff".parse().expect("Invalid router");
    
    // Subscribe to pending transactions
    let mut stream = web3.eth_subscribe()
                       .subscribe_new_pending_transactions()
                       .await.expect("Subscription failed");

    println!("Subscribed to pending transactions...");
    
    // Process each transaction
    while let Some(tx_hash) = stream.next().await {
        let web3_clone = web3.clone();
        tokio::spawn(process_transaction(web3_clone, tx_hash, router_address));             
    }  
}

async fn process_transaction(web3: Web3<WebSocket>, tx_hash_result: Result<H256, web3::Error>, router_address: H160) -> web3::Result<()> {
    let tx_hash = tx_hash_result?;
    let tx = web3.eth().transaction(TransactionId::Hash(tx_hash)).await?.ok_or_else(|| web3::Error::InvalidResponse(String::from("Transaction not found")))?;

    let from = tx.from.unwrap_or_else(|| H160::repeat_byte(0));
    let to = tx.to.unwrap_or_else(|| H160::repeat_byte(0));
    let block_number = tx.block_number.unwrap_or_else(|| U64::zero());

    if to == router_address {
        println!("Transaction to router detected: from: {:#x}, to: {:#x}, block number: {}", from, to, block_number);
        // Add further transaction processing logic here
    }
    Ok(())
}

// Load contract ABI from file
fn load_abi() -> web3::Result<Vec<u8>> {
    let mut file = File::open("src/abi/quickswapRouterV2.json")?;  
    let mut abi = Vec::new();
    file.read_to_end(&mut abi)?;
    Ok(abi)
}

async fn frontrun(web3: &Web3<WebSocket>) {
    println!("Front running...");
    // TODO: add front run
}