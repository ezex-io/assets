# Chain YAML Template  

Use the following YAML structure when submitting a new chain:  

```yaml
name: "<Chain Name>"
website: "<Official Website URL>"
description: "<Brief description of the blockchain>"
explorer: "<Block Explorer URL>"
symbol: "<Native Token Symbol>"
asset_type: "coin"
decimals: <Number of Decimals>
links:
  - name: "<Platform Name>"
    url: "<Profile URL>"
```

## Field Descriptions  

- **name**: The official name of the blockchain.  
- **website**: The official website URL of the blockchain.  
- **description**: A brief description of the blockchain and its purpose.  
- **explorer**: A link to the blockchain’s block explorer.  
- **symbol**: The symbol of the native token of the blockchain.  
- **asset_type**: Specifies whether the asset is a `"coin"` (native blockchain token).  
- **decimals**: The number of decimal places supported by the token.  
- **links**: A list of social or informational links related to the blockchain. Examples:  
  - **GitHub**: Repository link for development.  
  - **LinkedIn**: Official LinkedIn profile.  
  - **Twitter**: Official Twitter account.  

### Example Submission  

```yaml
name: Pactus
website: https://pactus.org
description: Pactus is an open source, layer-1 blockchain technology with a revolutionary,
  secure, Solid State Proof of Stake Consensus.
explorer: https://pacviewer.com/
symbol: PAC
asset_type: coin
decimals: 9
links:
- name: github
  url: https://github.com/pactus-project
- name: twitter
  url: https://twitter.com/pactuschain/
- name: telegram
  url: https://t.me/pactuschat
- name: telegram_news
  url: https://t.me/pactusblockchain
- name: facebook
  url: https://facebook.com/PactusChain
- name: coingecko
  url: https://coingecko.com/en/coins/pactus
- name: source_code
  url: https://github.com/pactus-project/pactus
- name: whitepaper
  url: https://github.com/pactus-project/Whitepaper/releases/latest/download/pactus_whitepaper.pdf
```

## Contribution Guidelines  

1. Fork this repository.  
2. Create a new YAML file under the `chains/{chain_name}` directory (e.g., `info.yml`).  
3. Follow the template structure and fill in all required details.  
4. Submit a pull request (PR) with the new chain information.  

For any questions or issues, feel free to open an issue in this repository.
