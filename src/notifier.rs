use std::env;

use anyhow::Result;
use reqwest::Client;
use solana_sdk::message;

use crate::{
    global::{BOT_TOKEN, CHAT_ID, NOTIFIED_TOKENS, SOL_PRICE},
    metadata::get_token_metadata,
    models::{TokenMeta, TradeData},
};

const SOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

pub async fn send_message(token: &String, amm: String, market_cap: f64) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", BOT_TOKEN.as_str());

    let dexscreener_link = format!("https://dexscreener.com/solana/{}", amm.to_lowercase());
    let message = format!(
        "🚀 New token alert 🚀\n\nToken: {}\nAMM: {}\nMarket Cap: ${:.2}M\n\nDexscreener: {}",
        token, amm, market_cap / 1_000_000.0, dexscreener_link
    );

    let client = Client::new();
    let resp = client
        .post(&url)
        .form(&[("chat_id", CHAT_ID.as_str()), ("text", &message)])
        .send()
        .await?;

    if resp.status().is_success() {
        println!("Message sent: {}", message);
    } else {
        eprintln!("Failed to send message: {:?}", resp.text().await?);
    }

    Ok(())
}

pub async fn process_trade(trade: TradeData) {
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

    if let Some(data) = get_token_metadata(&token, trade.pool_address.clone()).await {
        let sol_price = {
            let r = SOL_PRICE.read().unwrap();
            *r
        };
        let usd_price = token_price * sol_price;
        let market_cap = usd_price * data.supply;

        if market_cap > 10_000_000.0 {
            let already_notified = {
                let read_guard = NOTIFIED_TOKENS.read().unwrap();
                read_guard.contains(&token)
            };

            if !already_notified {
                let _ = send_message(&token, trade.pool_address.clone(), market_cap).await;

                let mut write_guard = NOTIFIED_TOKENS.write().unwrap();
                write_guard.insert(token);
            }
        }
    }
}
