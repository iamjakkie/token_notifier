use std::str::FromStr;
use crate::global::RPC_CLIENT;
use solana_account_decoder_client_types::token::UiTokenAmount;
use solana_client::rpc_response::{Response};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel}, pubkey::Pubkey,
};

const COMMITMENT_CONFIG: CommitmentConfig = CommitmentConfig::confirmed();

pub async fn fetch_token_metadata(mint: &String) -> Option<Response<UiTokenAmount>>{
    let rpc_client = RPC_CLIENT.clone();
    let pubkey = Pubkey::from_str(&mint).unwrap();

    match rpc_client.get_token_supply_with_commitment(&pubkey, COMMITMENT_CONFIG) {
        Ok(data) => {
            Some(data)
        },
        _ => {
            None
        }
    }
}