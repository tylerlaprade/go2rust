package dep

type LoadMode int

const (
	NeedName LoadMode = 1 << iota
	NeedFiles
	NeedTypes
)

type Config struct {
	Mode LoadMode
}

func Enabled(cfg *Config, bit LoadMode) bool {
	return cfg.Mode&bit != 0
}
