package main

import (
	"strings"
	"testing"
)

func TestReadsOfOneVariableInOneStatementEndTheirLocks(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type P struct{ X, Y int }

func two(a uint64, b bool) uint64 { return a }
func count(s string, n int) int    { return n }

func main() {
	go func() {}()
	x := 3
	s := "go"
	_ = two(uint64(x), x < 0)
	_ = count(s, len(s))
	_ = []int{-x, x}
	_ = P{x, -x}
}
`)
	// Each read is a lock; a lock held by a temporary until the statement
	// ends made the next read of the same variable wait forever.
	for _, want := range []string{
		"({ let __v = (*x.lock().unwrap().as_ref().unwrap()); __v }) as u64",
		"({ let __len = (*s.lock().unwrap().as_ref().unwrap()).len(); __len })",
		"-(({ let __v = (*x.lock().unwrap().as_ref().unwrap()); __v }))",
		"({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v })",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Some((*x.lock().unwrap().as_ref().unwrap()) as u64)") {
		t.Errorf("a conversion still holds the lock until the statement ends:\n%s", rust)
	}
}

func TestLenOfAStringConcatenation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	s := "go"
	_ = len(s + "!")
}
`)
	if !strings.Contains(rust, `(format!("{}{}", (*s.borrow().as_ref().unwrap()), "!".to_string())).len()`) {
		t.Errorf("len of a concatenation should measure the new String:\n%s", rust)
	}
}
