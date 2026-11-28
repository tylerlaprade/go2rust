package dep

func Get() any {
	xs := []any{1, "a"}
	var v any = xs
	return v
}
