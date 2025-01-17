mod global;
mod metadata;
mod models;
mod pricer;
mod rpc_client;

use zmq;

use crate::models::TradeData;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = zmq::Context::new();
    let subscriber = ctx.socket(zmq::SUB)?;
    subscriber.connect("tcp://localhost:5555")?;
    subscriber.set_subscribe(b"")?; // Subscribe to all topics

    loop {
        // Topic frame (if you used it)
        let _topic = subscriber.recv_string(0)?;
        // Actual data
        let msg = subscriber.recv_string(0)?;

        if let Ok(data) = msg {
            // 5. Deserialize JSON
            let trades: Vec<TradeData> = serde_json::from_str(&data)?;

            trades.iter().for_each(|trade| {
                println!("{:?}", trade);
            });

            // Now you can process the trades in your Market Cap watcher, metrics, etc.
        }
    }
}