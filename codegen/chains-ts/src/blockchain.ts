export interface Blockchain {
    readonly name: string;
    readonly description: string;
    readonly website: string;
    readonly explorer: string;
    readonly logo: string;
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