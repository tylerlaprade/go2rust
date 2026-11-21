package main

import (
	"strings"
	"testing"
)

func TestNumericConversionOnComparisonLeftIsParenthesized(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const max = 0xDFFF

func f(i uint32) bool {
	return uint32(max) < i
}
`)

	if !strings.Contains(rust, "as u32)) <") {
		t.Fatalf("Rust must not parse a conversion on the left of < as generic arguments:\n%s", rust)
	}
}
