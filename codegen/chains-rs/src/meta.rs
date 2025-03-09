/*
MIT License

Copyright (c) 2025 ezeX

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/     
        
// Code generated automatically. DO NOT EDIT.

use std::collections::HashMap;
use std::sync::Arc;
use crate::blockchain::Blockchain;
use crate::binance::BinanceBlockchain;
use crate::bitcoin::BitcoinBlockchain;
use crate::pactus::PactusBlockchain;

pub fn get_blockchains() -> HashMap<String, Arc<dyn Blockchain>> {
    let mut map: HashMap<String, Arc<dyn Blockchain>> = HashMap::new();
    map.insert("binance".to_string(), Arc::new(BinanceBlockchain {}));
    map.insert("bitcoin".to_string(), Arc::new(BitcoinBlockchain {}));
    map.insert("pactus".to_string(), Arc::new(PactusBlockchain {}));
    map
}
