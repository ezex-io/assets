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
    private readonly name: string;
    private readonly description: string;
    private readonly website: string;
    private readonly explorer: string;
    private readonly logo: string;
    private readonly links: Link[];
    private readonly assets: Asset[];

    constructor(data: BlockchainData) {
        this.name = data.name;
        this.description = data.description;
        this.website = data.website;
        this.explorer = data.explorer;
        this.logo = data.logo;
        this.links = data.links;
        this.assets = data.assets;
    }

    getName() {
        return this.name;
    }

    getDescription() {
        return this.description;
    }

    getWebsite() {
        return this.website;
    }

    getExplorer() {
        return this.explorer;
    }

    getLogo() {
        return this.logo;
    }

    getLinks() {
        return this.links;
    }

    getAssets() {
        return this.assets;
    }
}

export class Asset {
    private readonly id: string;
    private readonly name: string;
    private readonly address: string;
    private readonly symbol: string;
    private readonly type: string;
    private readonly assetType: string;
    private readonly bip44CoinType: number;
    private readonly description: string;
    private readonly website: string;
    private readonly explorer: string;
    private readonly decimals: number;
    private readonly icon: string;

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

    getId() {
        return this.id;
    }

    getName() {
        return this.name;
    }

    getAddress() {
        return this.address;
    }

    getSymbol() {
        return this.symbol;
    }

    getType() {
        return this.type;
    }

    getAssetType() {
        return this.assetType;
    }

    getBip44CoinType() {
        return this.bip44CoinType;
    }

    getDescription() {
        return this.description;
    }

    getWebsite() {
        return this.website;
    }

    getExplorer() {
        return this.explorer;
    }

    getDecimals() {
        return this.decimals;
    }

    getIcon() {
        return this.icon;
    }
}

export class Link {
    private readonly name: string;
    private readonly url: string;

    constructor(data: LinkData) {
        this.name = data.name;
        this.url = data.url;
    }

    getName() {
        return this.name;
    }

    getUrl() {
        return this.url;
    }
}
