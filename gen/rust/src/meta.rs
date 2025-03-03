// Code generated automatically. DO NOT EDIT.

use std::collections::HashMap;
use std::sync::Arc;
use crate::blockchain::Blockchain;

pub fn get_blockchains() -> HashMap<String, Arc<dyn Blockchain>> {
    let mut map: HashMap<String, Arc<dyn Blockchain>> = HashMap::new();
    map.insert("binance".to_string(), Arc::new(BinanceBlockchain));
    map.insert("bitcoin".to_string(), Arc::new(BitcoinBlockchain));
    map.insert("pactus".to_string(), Arc::new(PactusBlockchain));
    map
}
