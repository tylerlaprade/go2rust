package main

import (
	"fmt"
	"sort"
)

type Tally map[string][]int

func (t Tally) Add(name string, value int) {
	t[name] = append(t[name], value)
}

func (t Tally) Replace(old, new string) {
	if list, ok := t[old]; ok {
		delete(t, old)
		t[new] = append(t[new], list...)
	}
}

func main() {
	t := make(Tally)
	t.Add("a", 1)
	t.Add("a", 2)
	t.Add("b", 3)
	t.Replace("a", "c")

	keys := make([]string, 0, len(t))
	for k := range t {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Println(k, t[k])
	}
}
