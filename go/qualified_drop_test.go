package main

import (
	"strings"
	"testing"
)

// The generated code releases its lock guards with std::mem::drop, so a Go
// variable or function named drop does not shadow it.
func TestGeneratedGuardReleaseIsQualified(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func drop(n int) int { return n }

func main() {
	drop := []int{1, 2, 3}
	for _, v := range drop {
		fmt.Println(v)
	}
}
`)
	if !strings.Contains(rust, "std::mem::drop(__range_guard)") {
		t.Errorf("the range snapshot should release its guard with std::mem::drop:\n%s", rust)
	}
	if strings.Contains(rust, "; drop(__range_guard)") {
		t.Errorf("a bare drop( can be shadowed by the Go name drop:\n%s", rust)
	}
}
