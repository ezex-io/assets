# Token YAML Template  

Use the following YAML structure when submitting a new token:  

```yaml
name: "<Token Name>"
symbol: "<Token Symbol>"
asset_type: "token"
type: "<Token Standard (e.g., ERC20, BEP20, TRC20, etc.)>"
description: "<Brief description of the token>"
website: "<Official Website URL>"
explorer: "<Token Explorer URL>"
address: "<Smart Contract Address>"
```

## Field Descriptions  

- **name**: The official name of the token.  
- **symbol**: The token symbol (e.g., "USDT", "DAI").  
- **asset_type**: Must be set to `"token"` to indicate it's a non-native asset.  
- **type**: The token standard (e.g., ERC20 for Ethereum, BEP20 for Binance Smart Chain, TRC20 for Tron, etc.).  
- **description**: A brief description of the token and its purpose.  
- **website**: The official website URL of the token.  
- **explorer**: A link to the token’s block explorer or contract details page.  
- **address**: The smart contract address of the token on the blockchain.  

### Example Submission  

```yaml
name: "Tether USD"
symbol: "USDT"
asset_type: "token"
type: "ERC20"
description: "Tether (USDT) is a stablecoin pegged to the US Dollar."
website: "https://tether.to/"
explorer: "https://etherscan.io/token/0xdac17f958d2ee523a2206206994597c13d831ec7"
address: "0xdac17f958d2ee523a2206206994597c13d831ec7"
```

## Contribution Guidelines  

1. Fork this repository.  
2. Create a new YAML file under the `chains/{chain_name}/tokens/{symbol}` directory (e.g., `info.yml`).  
3. Follow the template structure and fill in all required details.  
4. Submit a pull request (PR) with the new token information.  

For any questions or issues, feel free to open an issue in this repository.  
