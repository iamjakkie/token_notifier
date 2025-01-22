use std::{env, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use solana_sdk::message;
use tokio::time::{sleep, Duration};

use crate::{
    global::{BOT_TOKEN, CHAT_ID, MESSAGE_QUEUE, NOTIFIED_TOKENS, SOLSCAN_API_KEY, SOL_PRICE}, metadata::get_token_meta, models::{TokenData, TokenMeta, TokenMetaResponse, TradeData}
};

const SOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

pub async fn send_message(msg: &String) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", BOT_TOKEN.as_str());

    let client = Client::new();
    let resp = client
        .post(&url)
        .form(&[("chat_id", CHAT_ID.as_str()), ("text", msg)])
        .send()
        .await?;

    if resp.status().is_success() {
        println!("Message sent: {}", msg);
    } else {
        eprintln!("Failed to send message: {:?}", resp.text().await?);
    }

    Ok(())
}

pub async fn process_trade(trade: TradeData) {
    if trade.base_mint != SOL_ADDRESS && trade.quote_mint != SOL_ADDRESS {
        return;
    }

    let (token, token_price) = if trade.base_mint != SOL_ADDRESS {
        (
            trade.base_mint,
            (trade.quote_amount / trade.base_amount).abs(),
        )
    } else {
        (
            trade.quote_mint,
            (trade.base_amount / trade.quote_amount).abs(),
        )
    };

    if let Ok(data) = get_token_meta(&token, &trade.pool_address).await {
        let sol_price = {
            let r = SOL_PRICE.read().unwrap();
            *r
        };
        let usd_price = token_price * sol_price;
        let market_cap = usd_price * data.supply;

        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let created_time = data.created_time;

        let token_age = if created_time > now_secs {
            0
        } else {
            let age_secs = now_secs - created_time;
            (age_secs as f64 / 86_400.0) as i32
        };

        if market_cap > 10_000_000.0 && token_age <= 7{
            let already_notified = {
                let read_guard = NOTIFIED_TOKENS.read().unwrap();
                read_guard.contains(&token)
            };

            if !already_notified {
                let _ = enqueue_message(generate_msg(data.name, token_age, data.holder, data.volume_24h, trade.pool_address.clone(), market_cap));

                let mut write_guard = NOTIFIED_TOKENS.write().unwrap();
                write_guard.insert(token);
            }
        }
    }
}

pub fn enqueue_message(msg: String) {
    let mut lock = MESSAGE_QUEUE.lock().unwrap();
    lock.push_back(msg);
}

pub fn generate_msg(name: String, age: i32, holders: u64, volume24h: u64, amm: String, market_cap: f64) -> String {
    let dexscreener_link = format!("https://dexscreener.com/solana/{}", amm.to_lowercase());
    format!(
        "🚀 Token: {}, created: {} days ago, holders: {}, volume24h: {}, MCAP: {}, Dexscreener: {}",
        name, age, holders, volume24h, (market_cap / 1_000_000.0), dexscreener_link
    )
}

pub async fn start_batching_sender() {
    let client = Client::new();

    loop {
        // 1. Wait for a short window to collect messages
        let wait_duration = Duration::from_secs(2); 
        sleep(wait_duration).await;

        // 2. Drain all messages currently in queue
        let mut msgs = Vec::new();
        {
            let mut lock = MESSAGE_QUEUE.lock().unwrap();
            while let Some(m) = lock.pop_front() {
                msgs.push(m);
            }
        }

        // If no messages, keep looping
        if msgs.is_empty() {
            continue;
        }

        // 3. Combine all messages into one
        let combined_text = msgs.join("\n");

        // 4. Send to Telegram
        if let Err(e) = send_message(&combined_text).await {
            eprintln!("Failed to send combined message: {:?}", e);
        } else {
            println!("Sent combined message with {} sub-messages", msgs.len());
        }
    }
}

