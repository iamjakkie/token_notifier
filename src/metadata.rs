use crate::{global::TOKEN_META_MAP, models::TokenMeta, rpc_client::fetch_token_metadata};

// fetch metadata
// update in memory metadata
// periodically dump metadata to db


pub async fn get_token_metadata(mint: &String, amm: String) -> Option<TokenMeta>{
    {
        // 1. Check cache first (read lock)
        let read_map = TOKEN_META_MAP.read().unwrap();
        if let Some(meta) = read_map.get(mint) {
            // Found in cache, return a clone
            return Some(meta.clone());
        }
    } // read lock scope ends

    // 2. If not found, fetch from the network
    if let Some(data) = fetch_token_metadata(mint).await {
        let decimals = data.value.decimals;
        let supply_str = data.value.ui_amount_string;
        let supply: f64 = supply_str.parse().unwrap_or(0.0);

        let token_meta = TokenMeta {
            mint: mint.to_string(),
            decimals,
            supply,
            amm: amm.to_string(),
        };

        {
            // 3. Insert into cache (write lock)
            let mut write_map = TOKEN_META_MAP.write().unwrap();
            write_map.insert(mint.to_string(), token_meta.clone());
        }

        Some(token_meta)
    } else {
        None
    }
}