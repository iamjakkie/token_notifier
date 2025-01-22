use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMeta {
    pub mint: String,
    pub decimals: u8,
    pub supply: f64,
    pub amm: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceResponse {
    pub id: String,
    pub success: bool,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenList(pub Vec<String>);

#[derive(Debug, Deserialize)]
pub struct TokenMetaResponse {
    pub success: bool,
    pub data: TokenData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenData {
    pub supply: f64,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub holder: u64,
    pub created_time: u64,
    pub first_mint_time: u64,
    pub volume_24h: u64,
    pub amm: String,
}