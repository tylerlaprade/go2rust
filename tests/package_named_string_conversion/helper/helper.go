package helper

type Path string

func Text() string {
	return ""
}

func Object(p Path) string {
	if p == "" {
		return "empty"
	}
	return string(p)
}
