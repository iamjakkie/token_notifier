use crate::{metadata::get_token_metadata, models::{TokenMeta, TradeData}};

const SOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

pub async fn send_message(message: String) {
    println!("Message sent: {}", message);
}

pub async fn process_trade(trade: TradeData) {
    // check if metadata in memory
    // if not go fetch it
    let token = if trade.base_mint != SOL_ADDRESS{
        trade.base_mint
    } else {
        trade.quote_mint
    };

    match get_token_metadata(&token, trade.pool_address).await {
        Some(data) => {
            
        },
        _ => {
            return;
        }
    }

}