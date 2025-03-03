# **Chains**
The **Chains** repository contains a structured list of blockchain networks and their associated tokens.

## **Generating a Blockchain**
To generate a Go implementation for a specific blockchain, run the following command:

```shell
python3 generator/generator.py --chain chains/bitcoin
```

Replace `chains/bitcoin` with the path to the blockchain you want to generate.

## **Project Structure**
```
├── chains
│   ├── binance
│   │   ├── assets
│   │   │   ├── bnb
│   │   │   │   ├── icon.svg
│   │   │   │   └── info.yml
│   │   │   └── usdt
│   │   │       ├── icon.svg
│   │   │       └── info.yml
│   │   ├── info.yml
│   │   └── logo.svg
│   ├── bitcoin
│   │   ├── assets
│   │   │   └── btc
│   │   │       ├── icon.svg
│   │   │       └── info.yml
│   │   ├── info.yml
│   │   └── logo.svg
│   └── pactus
│       ├── assets
│       │   └── pac
│       │       ├── icon.svg
│       │       └── info.yml
│       ├── info.yml
│       └── logo.svg
├── gen
│   └── golang
│       ├── go.mod
│       ├── interface.go
│       ├── pactus.gen.go
│       └── types.go
├── generator
│   └── templates
│       └── go.tmpl
├── README.md
└── template
    ├── asset
    │   ├── info.yml
    │   └── README.md
    └── chain
        ├── info.yml
        └── README.md
```

## **How It Works**
1. The script scans the `chains/` directory for the specified blockchain.
2. It reads `info.yml` files for the blockchain and its assets.
3. It encodes `logo.svg` and `icon.svg` into Go-friendly formats.
4. It generates a `.gen.go` file inside `gen/golang/`.

## **Generating All Chains**
To generate Go files for **all** blockchains, run:
```shell
for chain in chains/*; do
    python3 generator/generator.py --chain "$chain"
done
```
