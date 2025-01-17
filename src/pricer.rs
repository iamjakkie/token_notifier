use reqwest::Client;
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use anyhow::Result;

use crate::{global::SOL_PRICE, models::PriceResponse};

#[derive(Deserialize, Debug)]
struct SolanaPriceResponse {
    sol_price_usd: f64, // or name it to match the API response
}

async fn fetch_sol_price() -> Result<f64> {
    let url = "https://api-v3.raydium.io/mint/price?mints=So11111111111111111111111111111111111111112";
    let client = Client::new();

    let resp = client
        .get(url)
        .send()
        .await?
        .json::<PriceResponse>()
        .await?;

    let price: f64 = resp.data.get("So11111111111111111111111111111111111111112").expect("No price found").parse().unwrap();
    
    // Return the numeric price
    Ok(price)
}

pub async fn start_sol_price_updater() {
    // Spawn a background task so it runs indefinitely.
    tokio::spawn(async {
        loop {
            match fetch_sol_price().await {
                Ok(new_price) => {
                    // Update the global
                    let mut w = SOL_PRICE.write().unwrap();
                    *w = new_price;
                    println!("Updated SOL price: {}", new_price);
                }
                Err(e) => {
                    eprintln!("Failed to fetch SOL price: {:?}", e);
                }
            }

            // Wait 30s before the next update
            sleep(Duration::from_secs(30)).await;
        }
    });
}