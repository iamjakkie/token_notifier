use crate::{models::TokenMeta, rpc_client::fetch_token_metadata};

// fetch metadata
// update in memory metadata
// periodically dump metadata to db


pub async fn get_token_metadata(mint: &String, amm: String) -> Option<TokenMeta>{
    // to be changed
    if true {
        let metadata = fetch_token_metadata(mint).await;
        match metadata {
            Some(data) => {
                let values = data.value;
                let decimals = values.decimals;
                let supply_str = values.ui_amount_string;
                let supply: f64 = supply_str.parse().unwrap();
                Some(TokenMeta {
                    mint: mint.clone(),
                    decimals: decimals,
                    supply: supply,
                    amm: amm,
                })
            },
            _ => {
                None
            }
        }
    } else {
        None
    }
}