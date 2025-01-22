use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;

use crate::{global::{SOLSCAN_API_KEY, TOKEN_META_MAP}, models::{TokenData, TokenMeta}, rpc_client::fetch_token_metadata};

// fetch metadata
// update in memory metadata
// periodically dump metadata to db


// pub async fn get_token_metadata(mint: &String, amm: String) -> Option<TokenMeta>{
//     {
//         // 1. Check cache first (read lock)
//         let read_map = TOKEN_META_MAP.read().unwrap();
//         if let Some(meta) = read_map.get(mint) {
//             // Found in cache, return a clone
//             return Some(meta.clone());
//         }
//     } // read lock scope ends

//     // 2. If not found, fetch from the network
//     if let Some(data) = fetch_token_metadata(mint).await {
//         let decimals = data.value.decimals;
//         let supply_str = data.value.ui_amount_string;
//         let supply: f64 = supply_str.parse().unwrap_or(0.0);

//         let token_meta = TokenMeta {
//             mint: mint.to_string(),
//             decimals,
//             supply,
//             amm: amm.to_string(),
//         };

//         {
//             // 3. Insert into cache (write lock)
//             let mut write_map = TOKEN_META_MAP.write().unwrap();
//             write_map.insert(mint.to_string(), token_meta.clone());
//         }

//         Some(token_meta)
//     } else {
//         None
//     }
// }

pub async fn get_token_meta(mint: &String, amm: &String) -> Result<TokenData> {

    {
        //TODO: ideally check how stale the data is, and only fetch if it's not too old
        // 1. Check cache first (read lock)
        let read_map = TOKEN_META_MAP.read().unwrap();
        if let Some(meta) = read_map.get(mint) {
            // Found in cache, return a clone
            return Ok(meta.clone());
        }
    } // read lock scope ends


    let url = format!(
        "https://pro-api.solscan.io/v2.0/token/meta?address={}",
        mint
    );

    let client = Client::new();

    let response = client
        .get(&url)
        .header("token", SOLSCAN_API_KEY.to_string())
        .send()
        .await?;

    let status = response.status().clone();

    // You can check for non-200 statuses if you want more robust error handling
    if !status.is_success() {
        let err_text = response.text().await?;
        return Err(anyhow!("Solscan error: HTTP {} => {}", status, err_text));
    }

    // Parse JSON into our struct
    let response_json: Value = response.json().await?;
    let metadata = response_json.get("data").expect("No data field in response");

    let supply = metadata
        .get("supply")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .parse::<u64>().unwrap_or(0);

    let name = metadata
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();


    let symbol = metadata
        .get("symbol")
        .and_then(|v| v.as_str())
        .unwrap_or("???")
        .to_string();

    let decimals = metadata
        .get("decimals")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u8;

    let holder = metadata
        .get("holder")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let created_time = metadata
        .get("created_time")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let first_mint_time = metadata
        .get("first_mint_time")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let volume_24h = metadata
        .get("volume_24h")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let calculated_supply: f64 = supply as f64 / 10u64.pow(decimals as u32) as f64;

    let token_data = TokenData {
        supply: calculated_supply,
        name,
        symbol,
        decimals,
        holder,
        created_time,
        first_mint_time,
        volume_24h,
        amm: amm.to_string(),
    };

    {
        let mut write_map = TOKEN_META_MAP.write().unwrap();
        write_map.insert(mint.to_string(), token_data.clone());
    }

    Ok(token_data)
}