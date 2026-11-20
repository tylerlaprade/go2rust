package main

import (
	"strings"
	"testing"
)

func TestRangeRuneArithmeticUsesI32(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func digits(s string) int {
	n := 0
	for _, c := range s {
		n = n*10 + int(c-'0')
		_ = 'a' + (c-'a'+1)%26
	}
	return n
}
`)
	for _, want := range []string{
		"((c as i32) - ('0' as i32))",
		"('a' as i32) + ((c as i32) - ('a' as i32) + (1 as i32)) % (26 as i32)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("arithmetic on a range rune should compute in i32, missing %q:\n%s", want, rust)
		}
	}
}
