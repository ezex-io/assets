package golang

type (
	AssetType string
)

const (
	Coin  = "coin"
	Token = "token"
)

type Link struct {
	Name string
	URL  string
}
