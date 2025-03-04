export interface Blockchain {
    name: string;
    description: string;
    website: string;
    explorer: string;
    symbol: string;
    decimals: number;
    logo: string;
    links: Link[];
    assets: Asset[];
}

export interface Asset {
    id: string;
    name: string;
    symbol: string;
    type: string;
    assetType: string;
    description: string;
    website: string;
    explorer: string;
    decimals: number;
    status: string;
    icon: string;
}

export interface Link {
    name: string;
    url: string;
}