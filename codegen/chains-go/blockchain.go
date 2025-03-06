package chains

type Blockchain interface {
	Name() string
	Description() string
	Website() string
	Explorer() string
	Links() []Link
	Assets() []Asset
	Asset(id string) Asset
	// Logo return raw svg as string
	Logo() string
}
