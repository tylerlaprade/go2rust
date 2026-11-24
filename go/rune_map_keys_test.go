package main

import (
	"strings"
	"testing"
)

// A rune from ranging over a string is a Rust char; as the key of a
// map[rune]V it is the map's i32 (DEV-179).
func TestStringRangeRuneIsMapRuneKey(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	freq := map[rune]int{}
	seen := map[rune]bool{}
	for _, r := range "hello" {
		freq[r]++
		if !seen[r] {
			seen[r] = true
		}
	}
	fmt.Println(freq, seen)
}
`)
	for _, want := range []string{
		"let __key = r as i32;",
		"__map.get(&(r as i32))",
		"let __map_key = r as i32;",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}

// %c of a rune that is not a Rust char, such as one from a []rune, prints
// the character with that code point.
func TestFormatCharOfRuneSliceElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	for _, r := range []rune("世界") {
		fmt.Printf("%c", r)
	}
}
`)
	if !strings.Contains(rust, "char::from_u32((r) as i64 as u32).unwrap_or('\\u{FFFD}')") {
		t.Errorf("%%c should print the rune's character:\n%s", rust)
	}
}
