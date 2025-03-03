package golang

type Blockchain interface {
	Name() string
	Description() string
	Website() string
	Explorer() string
	Symbol() string
	Decimals() uint
	Links() []Link
	Assets() []Asset
	Asset(id string) Asset
	// Logo return raw svg as string
	Logo() string
}

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
