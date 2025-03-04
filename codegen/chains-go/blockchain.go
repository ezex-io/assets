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
