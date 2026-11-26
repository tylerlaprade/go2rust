package dep

// Pick returns v.
func Pick[T any](v T) T { return v }

// Join joins a and b with sep between them.
func Join[A, B any](a A, b B, sep string) string {
	return toString(a) + sep + toString(b)
}

func toString(v any) string {
	switch x := v.(type) {
	case string:
		return x
	case int:
		if x == 0 {
			return "0"
		}
		digits := ""
		for x > 0 {
			digits = string(rune('0'+x%10)) + digits
			x /= 10
		}
		return digits
	}
	return "?"
}
