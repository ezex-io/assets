package golang

type Asset interface {
	ID() string
	Name() string
	Description() string
	Symbol() string
	Decimals() uint
	Type() string
	AssetType() AssetType
	Website() string
	Explorer() string
	Status() string
	// Icon return raw svg as string
	Icon() string
}
