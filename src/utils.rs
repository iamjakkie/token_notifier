use std::{collections::HashMap, fs};
use serde_json;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

use crate::{global::TOKEN_META_MAP, models::TokenMeta};

pub fn load_token_meta_cache(file_path: &str) {
    if let Ok(contents) = fs::read_to_string(file_path) {
        if let Ok(cache_map) = serde_json::from_str::<HashMap<String, TokenMeta>>(&contents) {
            let mut write_map = TOKEN_META_MAP.write().unwrap();
            *write_map = cache_map;
            println!("Loaded token metadata from {}", file_path);
        }
    }
}

pub fn dump_token_meta_cache(file_path: &str) {
    let read_map = TOKEN_META_MAP.read().unwrap();
    if let Ok(json_str) = serde_json::to_string_pretty(&*read_map) {
        if let Err(e) = fs::write(file_path, json_str) {
            eprintln!("Failed to write token meta cache: {}", e);
        }
    }
}

pub async fn start_dumping_task(file_path: String) {
    loop {
        dump_token_meta_cache(&file_path);
        sleep(Duration::from_secs(60)).await; // e.g., every 1 minute
    }
}