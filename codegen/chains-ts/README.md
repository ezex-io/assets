# chains-ts

## Overview

`chains-ts` is a TypeScript library for defining blockchain networks and their
associated assets. It provides a structured way to represent various blockchains
and their respective assets, enabling easy integration into cryptocurrency-related applications.

## Features

-   Standardized blockchain and asset definitions.
-   Support for multiple blockchain networks (e.g., Bitcoin, Binance, Pactus).
-   Extensible asset representation.
-   Designed for interoperability with Web3 and crypto applications.

## Installation

To install `chains-ts`, use npm or yarn:

```sh
npm install chains-ts
```

or

```sh
yarn add chains-ts
```

## Usage

Here's a basic example of how to use `chains-ts`:

```ts
import { BinanceBlockchain } from "chains-ts";

console.log(`Blockchain: ${BinanceBlockchain.getName()}`);
```

## Building the Project

To compile the TypeScript code:

```sh
npm run build
```
