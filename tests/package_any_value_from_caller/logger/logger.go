// Package logger prints whatever it is handed. It creates almost none of the
// values itself, so what it knows about their types comes from its callers.
package logger

import "fmt"

// Show prints one value.
func Show(v any) { fmt.Println(v) }

// Each prints every value on its own line.
func Each(args ...any) {
	for _, a := range args {
		fmt.Println(a)
	}
}

// Describe reports whether v is an error, and its message if it is.
func Describe(v any) string {
	if err, ok := v.(error); ok {
		return "error: " + err.Error()
	}
	return "not an error"
}

// RequireError returns the error from a one-result type assertion.
func RequireError(v any) error { return v.(error) }

// Classify exercises the type-switch path separately from comma-ok assertion.
func Classify(v any) string {
	switch err := v.(type) {
	case error:
		return "switch error: " + err.Error()
	default:
		return "switch other"
	}
}

// Kind names the type of the value v holds.
func Kind(v any) string { return fmt.Sprintf("%T", v) }

// Numbers hands back a value the caller did not create.
func Numbers() any { return []int64{10, 20} }
