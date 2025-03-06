interface BlockchainData {
    name: string;
    description: string;
    website: string;
    explorer: string;
    logo: string;
    links: Link[];
    assets: Asset[];
}

interface AssetData {
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

interface LinkData {
    name: string;
    url: string;
}

export class Blockchain {
    readonly name: string;
    readonly description: string;
    readonly website: string;
    readonly explorer: string;
    readonly logo: string;
    readonly links: Link[];
    readonly assets: Asset[];

    constructor(data: BlockchainData) {
        this.name = data.name;
        this.description = data.description;
        this.website = data.website;
        this.explorer = data.explorer;
        this.logo = data.logo;
        this.links = data.links;
        this.assets = data.assets;
    }
}

export class Asset {
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

    constructor(data: AssetData) {
        this.id = data.id;
        this.name = data.name;
        this.address = data.address;
        this.symbol = data.symbol;
        this.type = data.type;
        this.assetType = data.assetType;
        this.bip44CoinType = data.bip44CoinType;
        this.description = data.description;
        this.website = data.website;
        this.explorer = data.explorer;
        this.decimals = data.decimals;
        this.icon = data.icon;
    }
}

export class Link {
    readonly name: string;
    readonly url: string;

    constructor(data: LinkData) {
        this.name = data.name;
        this.url = data.url;
    }
}
