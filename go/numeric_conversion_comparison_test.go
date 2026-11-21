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

func TestConstCastOnLeftOfComparisonIsParenthesized(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const limit = 10

func above(value int32) bool {
	return limit < value
}
`)
	if !strings.Contains(rust, "(LIMIT as i32) <") {
		t.Fatalf("a cast on the left of < must be parenthesized for Rust to parse it as a comparison:\n%s", rust)
	}
}

func TestRangeIndexUntypedConstComparisonCastsAreParenthesized(t *testing.T) {
	rust := transpileRegression(t, `package main

func f(xs []int) bool {
	for i := range xs {
		if 10 < i {
			return true
		}
		if i < 10 {
			return true
		}
	}
	return false
}`, &TypeInfo{})

	if !strings.Contains(rust, "(10 as i64) < (i as i64)") {
		t.Fatalf("range-index comparison casts must be parenthesized so Rust parses < as comparison:\n%s", rust)
	}
}
