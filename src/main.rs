mod global;
mod metadata;
mod models;
mod notifier;
mod pricer;
mod rpc_client;
mod utils;

use std::sync::{Arc, RwLock};

use notifier::{process_trade, start_batching_sender};
use pricer::start_sol_price_updater;
use utils::{load_notified_tokens, load_token_meta_cache, start_dumping_notified_tokens, start_dumping_task};
use zmq;

use crate::models::TradeData;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    load_token_meta_cache("token_meta.json");
    load_notified_tokens("notified_tokens.json");

    tokio::spawn(async {
        start_dumping_task("token_meta.json".to_string()).await;
    });

    tokio::spawn(async move {
        start_dumping_notified_tokens("notified_tokens.json".to_string()).await;
    });
    
    tokio::spawn(async move {
        start_batching_sender().await;
    });

    
    start_sol_price_updater().await;

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    

    let ctx = zmq::Context::new();
    let subscriber = ctx.socket(zmq::SUB)?;
    subscriber.connect("tcp://localhost:5555")?;
    subscriber.set_subscribe(b"")?; // Subscribe to all topics

    loop {
        let _topic = subscriber.recv_string(0)?;
        // Actual data
        let msg = subscriber.recv_string(0)?;

        if let Ok(data) = msg {
            let trades: Vec<TradeData> = serde_json::from_str(&data)?;

            for trade in trades{
                tokio::spawn(async move{
                    process_trade(trade).await;
                });
            }
        }
    }

    // let trades: Vec<TradeData> = vec![
    //     TradeData {
    //         block_date: "2025-01-17".to_string(),
    //         block_time: 1737109270,
    //         block_slot: 314555166,
    //         signature: "55gPyGLvXZf41L5obSSCB3uHHmD6nNR3jWsnSuUZqsykyDewdk5XXm4chfJMJ3pVxPAEZK2Jz2kJvUiVe4K88ufD".to_string(),
    //         tx_id: "RpygY1y6tgZ4f5TYFZhiA9BBf3sqF795TTsYesrH7qbhLxNCckD7PvMm49NaS4Py5zrt5uvp3TAxQQe5wdRutfq7sQffjWurbNExGLdYpi3uUEMzXNxDJdF1".to_string(),
    //         signer: "CjgBUc9uoVV8khjRxmLVEqRSiNf6NvKUDeqcsxpSTpyt".to_string(),
    //         pool_address: "DPj8vWSHL7SM2SNsCsjmCZb9rKroDFPfn3Gait5eXDo9".to_string(),
    //         base_mint: "So11111111111111111111111111111111111111112".to_string(),
    //         quote_mint: "8zyGJ3DoJ1TfqxvQRFcQeiSUccymmXhsUVdGDeatpump".to_string(),
    //         base_vault: "EQK5xpXLnb1RBDJyDWRi7FKSPjNcdMZWcqVEm3Vhisqw".to_string(),
    //         quote_vault: "Bv34k1EiUae9LUot8QomioAgDveNS5L7Ejz2NQUMPvuP".to_string(),
    //         base_amount: -0.638837267,
    //         quote_amount: 687523.101823,
    //         is_inner_instruction: false,
    //         instruction_index: 3,
    //         instruction_type: "SwapBaseIn".to_string(),
    //         inner_instruction_index: 0,
    //         outer_program: "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string(),
    //         inner_program: "".to_string(),
    //         txn_fee_lamports: 305000,
    //         signer_lamports_change: -638837267,
    //     }
    // ];

    // for trade in trades{
    //     tokio::spawn(async move{
    //         process_trade(trade).await;
    //     });
    // }

    
}