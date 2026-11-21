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

type RuneLenTest struct {
	r    rune
	size int
}

var runelentests = []RuneLenTest{
	{0, 1},
	{'e', 1},
	{'é', 2},
	{'☺', 3},
	{RuneError, 3},
	{MaxRune, 4},
	{0xD800, -1},
	{0xDFFF, -1},
	{MaxRune + 1, -1},
	{-1, -1},
}

func TestRuneLen(t *testing.T) {
	for _, tt := range runelentests {
		if size := RuneLen(tt.r); size != tt.size {
			t.Errorf("RuneLen(%#U) = %d, want %d", tt.r, size, tt.size)
		}
	}
}

type ValidRuneTest struct {
	r  rune
	ok bool
}

var validrunetests = []ValidRuneTest{
	{0, true},
	{'e', true},
	{'é', true},
	{'☺', true},
	{RuneError, true},
	{MaxRune, true},
	{0xD7FF, true},
	{0xD800, false},
	{0xDFFF, false},
	{0xE000, true},
	{MaxRune + 1, false},
	{-1, false},
}

func TestValidRune(t *testing.T) {
	for _, tt := range validrunetests {
		if ok := ValidRune(tt.r); ok != tt.ok {
			t.Errorf("ValidRune(%#U) = %t, want %t", tt.r, ok, tt.ok)
		}
	}
}

func run(t *testing.T, name string, test func(*testing.T)) bool {
	t.Reset()
	test(t)
	if t.Failed() {
		println("FAIL " + name)
		return false
	}
	println("PASS " + name)
	return true
}

func main() {
	t := &testing.T{}
	passed := 0
	failed := 0
	for _, tt := range []struct {
		name string
		test func(*testing.T)
	}{
		{"TestConstants", TestConstants},
		{"TestRuneLen", TestRuneLen},
		{"TestValidRune", TestValidRune},
	} {
		if run(t, tt.name, tt.test) {
			passed++
		} else {
			failed++
		}
	}
	println("summary", passed, "passed,", failed, "failed")
}
