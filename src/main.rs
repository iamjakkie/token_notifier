use serde::{Deserialize, Serialize};
use zmq;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TradeData {
    pub block_date: String,
    pub block_time: i64,
    pub block_slot: u64,
    pub signature: String,
    pub tx_id: String,
    pub signer: String,
    pub pool_address: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub base_amount: f64,
    pub quote_amount: f64,
    pub is_inner_instruction: bool,
    pub instruction_index: u32,
    pub instruction_type: String,
    pub inner_instruction_index: u32,
    pub outer_program: String,
    pub inner_program: String,
    pub txn_fee_lamports: u64,
    pub signer_lamports_change: i64,
}


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

            println!("Received trades: {:#?}", trades);

            // Now you can process the trades in your Market Cap watcher, metrics, etc.
        }
    }
}