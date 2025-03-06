package chains

type Asset interface {
	ID() string
	Name() string
	// Address is for token
	Address() string
	Description() string
	Symbol() string
	Decimals() uint
	Type() string
	AssetType() AssetType
	BIP44CoinType() int
	Website() string
	Explorer() string
	// Icon return raw svg as string
	Icon() string
}
