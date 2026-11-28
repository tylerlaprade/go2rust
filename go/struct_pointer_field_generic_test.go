package main

import (
	"strings"
	"testing"
)

// A generic comparable struct with a pointer field gets a hand-written
// PartialEq and Ord, which compare copies of its type-parameter fields with
// == and cmp. A derive would bound each parameter; the hand-written impls
// must bound them too, or they do not build (#354).
func TestGenericStructPointerFieldComparatorsBoundTheirParameters(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type K[T comparable] struct {
	P *int
	V T
}

func main() {
	x := 1
	m := map[K[int]]int{}
	m[K[int]{&x, 1}]++
	fmt.Println(K[int]{&x, 1} == K[int]{&x, 1}, len(m))
}
`)
	for _, want := range []string{
		"PartialEq for K<T> where T: PartialEq + Clone {",
		"Eq for K<T> where T: PartialEq + Clone {}",
		"PartialOrd for K<T> where T: Ord + Clone {",
		"Ord for K<T> where T: Ord + Clone {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected %q:\n%s", want, rust)
		}
	}
}

// A *M field, where M aliases sync.Mutex, is written as a pointer cell, not
// a bare GoMutex, so its comparators compare the cell as other pointers
// are compared.
func TestAliasedMutexPointerFieldComparesAsItsCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"sync"
)

type M = sync.Mutex

type L struct {
	Name string
	Mu   *M
}

func main() {
	var l L
	fmt.Println(l == L{}, len(map[L]int{l: 1}))
}
`)
	if strings.Contains(rust, "GoMutex::go_ptr_eq(&self.mu") || strings.Contains(rust, "self.mu.is_nil") {
		t.Fatalf("an aliased *sync.Mutex field should not compare as a bare GoMutex:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::ptr_eq(&self.mu, &other.mu)") {
		t.Fatalf("an aliased *sync.Mutex field should compare as its pointer cell:\n%s", rust)
	}
}
