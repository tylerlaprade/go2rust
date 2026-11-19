package main

import (
	"strings"
	"testing"
)

func TestTailLogicalReturnWithNilComparison(t *testing.T) {
	rust := transpileTypedRegression(t, `package main
func ready(p *bool) bool { return p != nil && *p }
func missing(p *bool) bool { return p == nil || *p }`)
	if strings.Count(rust, "return { let __nil_result") != 2 {
		t.Fatalf("both leading nil checks need expression context:\n%s", rust)
	}
}
