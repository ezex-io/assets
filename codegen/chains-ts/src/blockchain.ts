export interface Blockchain {
    name: string;
    description: string;
    website: string;
    explorer: string;
    logo: string;
    links: Link[];
    assets: Asset[];
}

export interface Asset {
    id: string;
    name: string;
    address: string;
    symbol: string;
    type: string;
    assetType: string;
    bip44CoinType: number;
    description: string;
    website: string;
    explorer: string;
    decimals: number;
    icon: string;
}

export interface Link {
    name: string;
    url: string;
}