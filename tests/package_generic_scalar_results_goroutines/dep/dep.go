package dep

// Pick returns v.
func Pick[T any](v T) T { return v }

// Size returns 1 whatever v is.
func Size[T any](v T) int { return 1 }

// Pair returns both values.
func Pair[A, B any](a A, b B) (A, B) { return a, b }

// Larger reports whether a is larger than b.
func Larger[T int | float64](a, b T) bool { return a > b }

// Max returns the larger of a and b.
func Max[T int | float64](a, b T) T {
	if a > b {
		return a
	}
	return b
}
