// Code generated automatically. DO NOT EDIT.

import { Blockchain } from "./blockchain";
import { BinanceBlockchain } from "./binance";
import { BitcoinBlockchain } from "./bitcoin";
import { PactusBlockchain } from "./pactus";

export const Blockchains: Record<string, Blockchain> = {
    "binance": new BinanceBlockchain(),
    "bitcoin": new BitcoinBlockchain(),
    "pactus": new PactusBlockchain(),
};
