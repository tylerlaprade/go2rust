// A map read out of a struct field into a local variable is the same map:
// writes through the variable show in the field, for a pointer and a value
// struct, with := and with var.
package main

import (
	"fmt"
	"sort"
)

type Store struct {
	data map[string]int
	tags map[string][]string
}

func keys(m map[string]int) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

func (s *Store) Data() map[string]int { return s.data }

func main() {
	st := &Store{data: map[string]int{"a": 1}, tags: map[string][]string{}}
	m := st.data
	m["b"] = 2
	d := st.Data()
	d["c"] = 3
	var v = st.data
	v["d"] = 4
	delete(m, "a")
	fmt.Println(keys(st.data), len(m), len(d), len(v))

	t := st.tags
	t["x"] = append(t["x"], "first")
	fmt.Println(st.tags)

	s2 := Store{data: map[string]int{}}
	m2 := s2.data
	m2["z"] = 1
	fmt.Println(keys(s2.data))

	// Rebinding the variable leaves the field alone.
	m = map[string]int{"new": 1}
	fmt.Println(keys(st.data), keys(m))
}
