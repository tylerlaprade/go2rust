package main

import (
	"oracle/testing"
	"unicode"
	. "unicode/utf8"
)

// TestConstants is copied verbatim from unicode/utf8's own utf8_test.go.
// Keeping the package test body unchanged makes this fixture an oracle for the
// source-transpiled package rather than a hand-written approximation of it.
func TestConstants(t *testing.T) {
	if MaxRune != unicode.MaxRune {
		t.Errorf("utf8.MaxRune is wrong: %x should be %x", MaxRune, unicode.MaxRune)
	}
	if RuneError != unicode.ReplacementChar {
		t.Errorf("utf8.RuneError is wrong: %x should be %x", RuneError, unicode.ReplacementChar)
	}
}

func main() {
	t := &testing.T{}
	TestConstants(t)
	if t.Failed() {
		println("FAIL TestConstants")
		println("summary 0 passed, 1 failed")
		return
	}
	println("PASS TestConstants")
	println("summary 1 passed, 0 failed")
}
