# **Chains**
The **Chains** repository contains a structured list of blockchain networks and their associated assets (tokens & coins).  
It supports **automatic code generation** for **Go** and **Rust** implementations.

---

## **📌 Generating a Blockchain**
To generate **Go** or **Rust** and **typescript** code for a specific blockchain, run:

```shell
python3 generator/generator.py --chain chains/bitcoin --lang go
```
```shell
python3 generator/generator.py --chain chains/bitcoin --lang rust
```
```shell
python3 generator/generator.py --chain chains/bitcoin --lang ts
```

Replace `chains/bitcoin` with the path to the blockchain you want to generate.

---

## **⚙️ How It Works**
1. The generator scans the **`chains/`** directory for the specified blockchain.
2. It reads **`info.yml`** files for the blockchain and its assets.
3. It encodes **`logo.svg`** and **`icon.svg`** into **Go-friendly** and **Rust-friendly** formats.
4. It generates:
    - **Go**: `gen/golang/{blockchain}.gen.go`
    - **Rust**: `gen/rust/src/{blockchain}.rs`

---

## **📌 Generating All Blockchains**
To generate **Go** files for all blockchains:
```shell
for chain in chains/*; do
    python3 generator/generator.py --chain "$chain" --lang go
done
```

To generate **Rust** files for all blockchains:
```shell
for chain in chains/*; do
    python3 generator/generator.py --chain "$chain" --lang rust
done
```

To generate **Typescript** files for all blockchains:
```shell
for chain in chains/*; do
    python3 generator/generator.py --chain "$chain" --lang ts
done
```
