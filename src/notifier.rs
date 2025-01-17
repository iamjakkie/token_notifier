use crate::{global::SOL_PRICE, metadata::get_token_metadata, models::{TokenMeta, TradeData}};

const SOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";

pub async fn send_message(message: String) {
    println!("Message sent: {}", message);
}

pub async fn process_trade(trade: TradeData) {
    // check if metadata in memory
    // if not go fetch it
    let (token, token_price) = if trade.base_mint != SOL_ADDRESS{
        (trade.base_mint, (trade.quote_amount/trade.base_amount).abs())
    } else {
        (trade.quote_mint, (trade.base_amount/trade.quote_amount).abs())
    };

    match get_token_metadata(&token, trade.pool_address).await {
        Some(data) => {
            let sol_price = {
                let r = SOL_PRICE.read().unwrap();
                *r
            };
            let usd_price = token_price * sol_price;
            let market_cap = usd_price * data.supply;
            // println!("Market Cap: {}", market_cap);
            if market_cap > 10_000_000.0 {
                send_message(format!("Market Cap of {} is {}", token, market_cap)).await;
            }
        },
        _ => {
            return;
        }
    }

}