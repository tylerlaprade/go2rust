// The usual generic helpers, Map, Filter, Reduce, Keys and Max, over
// slices and maps of numbers and strings, a Pair[K, V] with a String
// method and a Set[T] defined over a map. A []T was a slice of handles,
// so none of these compiled with a []int or []string (DEV-69). Generic
// declarations are written once per instantiation now.
package main

import (
	"cmp"
	"fmt"
	"sort"
	"strings"
)

type Pair[K comparable, V any] struct {
	Key K
	Val V
}

func (p Pair[K, V]) String() string { return fmt.Sprintf("%v=%v", p.Key, p.Val) }

type Set[T comparable] map[T]struct{}

func (s Set[T]) Add(v T)      { s[v] = struct{}{} }
func (s Set[T]) Has(v T) bool { _, ok := s[v]; return ok }
func (s Set[T]) Len() int     { return len(s) }

func Keys[K cmp.Ordered, V any](m map[K]V) []K {
	keys := make([]K, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
	return keys
}

func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func Filter[T any](xs []T, keep func(T) bool) []T {
	var out []T
	for _, x := range xs {
		if keep(x) {
			out = append(out, x)
		}
	}
	return out
}

func Reduce[T, A any](xs []T, init A, f func(A, T) A) A {
	acc := init
	for _, x := range xs {
		acc = f(acc, x)
	}
	return acc
}

func Map[T, U any](xs []T, f func(T) U) []U {
	out := make([]U, len(xs))
	for i, x := range xs {
		out[i] = f(x)
	}
	return out
}

func main() {
	ages := map[string]int{"bob": 30, "alice": 25, "carol": 35}
	var pairs []Pair[string, int]
	for _, k := range Keys(ages) {
		pairs = append(pairs, Pair[string, int]{k, ages[k]})
	}
	fmt.Println(pairs)
	fmt.Println(pairs[0], len(pairs))
	s := Set[string]{}
	for _, w := range strings.Fields("a b a c b") {
		s.Add(w)
	}
	fmt.Println(s.Len(), s.Has("a"), s.Has("z"))
	fmt.Println(Max(3, 7), Max(2.5, -1.0), Max("go", "rust"))
	evens := Filter([]int{1, 2, 3, 4, 5, 6}, func(n int) bool { return n%2 == 0 })
	fmt.Println(evens, Reduce(evens, 0, func(a, n int) int { return a + n }))
	words := Map([]int{1, 22, 333}, func(n int) string { return strings.Repeat("x", len(fmt.Sprint(n))) })
	fmt.Println(words, Reduce(words, "", func(a string, w string) string { return a + w + "|" }))
	ids := Keys(map[int]bool{3: true, 1: false, 2: true})
	fmt.Println(ids)
}
