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
    readonly id: string;
    readonly name: string;
    readonly address: string;
    readonly symbol: string;
    readonly type: string;
    readonly assetType: string;
    readonly bip44CoinType: number;
    readonly description: string;
    readonly website: string;
    readonly explorer: string;
    readonly decimals: number;
    readonly icon: string;
}

export interface Link {
    name: string;
    url: string;
}