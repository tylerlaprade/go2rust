package main

import (
	"strings"
	"testing"
)

// A type named like a binding in a generated helper, type count int against
// the WaitGroup helper's let count, takes another Rust name everywhere, so
// the helper's binding does not try to shadow the type's tuple struct (#335).
func TestTypeNamedLikeHelperBindingIsRenamed(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type count int

func run() count {
	var wg sync.WaitGroup
	var c count = 2
	wg.Add(1)
	go func() { defer wg.Done(); c++ }()
	wg.Wait()
	return c
}
`)
	if !strings.Contains(rust, "pub struct count_(") {
		t.Fatalf("the type should be written as count_:\n%s", rust)
	}
	if strings.Contains(rust, "pub struct count(") {
		t.Fatalf("a tuple struct named count would clash with the helper's let count:\n%s", rust)
	}
}
