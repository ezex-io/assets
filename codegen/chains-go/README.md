# chains-go

## Overview
`chains-go` is a Go library for defining blockchain networks and their associated assets. It provides a structured way to represent various blockchains and their respective assets, enabling easy integration into cryptocurrency-related applications.

## Features
- Standardized blockchain and asset definitions.
- Support for multiple blockchain networks (e.g., Bitcoin, Binance, Pactus).
- Extensible asset representation.
- Designed for interoperability with Web3 and crypto applications.

## Installation
To use `chains-go`, install it with:

```sh
go get github.com/ezex-io/chains/codegen/chains-go
```

## Usage
Here's a basic example of how to use `chains-go`:

```go
package main

import (
    "fmt"
    "github.com/ezex-io/chains/codegen/chains-go"
)

func main() {
    binance := chains.Binance()
    fmt.Println("Blockchain:", binance.Name())
}
```
