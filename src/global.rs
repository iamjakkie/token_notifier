use std::{collections::HashMap, env, sync::{Arc, RwLock}};
use solana_client::rpc_client::RpcClient;
use lazy_static::lazy_static;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::models::TokenMeta;

lazy_static! {
    pub static ref RPC_CLIENT: Arc<RpcClient> = {
        let rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL is not set");
        Arc::new(RpcClient::new_with_commitment(
            rpc_url,
            CommitmentConfig::confirmed(),
        ))
    };
}

lazy_static! {
    pub static ref SOL_PRICE: RwLock<f64> = RwLock::new(0.0);
}

lazy_static!{
    pub static ref BOT_TOKEN: String = env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    pub static ref CHAT_ID: String = env::var("CHAT_ID").expect("CHAT_ID is not set");
}

lazy_static! {
    pub static ref TOKEN_META_MAP: RwLock<HashMap<String, TokenMeta>> = RwLock::new(HashMap::new());
}