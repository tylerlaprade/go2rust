// A parallel assignment stores into map elements: it swaps slice, string and
// pointer values, adds new keys, and uses the key and the map as they were
// before the statement assigned anything.
package main

import (
	"fmt"
	"sort"
)

type Inventory struct {
	stock map[string]int
}

func keys(m map[string]int) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

func main() {
	mm := map[string][]int{"a": {1}, "b": {2}}
	mm["a"], mm["b"] = mm["b"], mm["a"]
	fmt.Println(mm)
	mm["a"][0] = 7
	fmt.Println(mm)

	ms := map[string]string{"a": "x", "b": "y"}
	ms["a"], ms["b"] = ms["b"], ms["a"]
	fmt.Println(ms)

	mp := map[int]*int{}
	one, two := 1, 2
	mp[1], mp[2] = &one, &two
	mp[1], mp[2] = mp[2], mp[1]
	*mp[1] = 20
	fmt.Println(*mp[1], *mp[2], one, two)

	counts := map[string]int{}
	total := 0
	for _, w := range []string{"x", "y", "x"} {
		counts[w], total = counts[w]+1, total+1
	}
	fmt.Println(counts, total)

	k := "p"
	m := map[string]int{"p": 1}
	k, m[k] = "q", 5
	fmt.Println(k, m)

	old := map[string]int{"apple": 3}
	inv := &Inventory{stock: old}
	inv.stock, inv.stock["pear"] = map[string]int{}, 9
	fmt.Println(keys(old), keys(inv.stock), old["pear"])

	// Map variables and fields swap handles: neither map is emptied.
	a := map[string]int{"a": 1}
	b := map[string]int{"b": 2}
	a, b = b, a
	a["a2"] = 1
	fmt.Println(keys(a), keys(b))
	inv.stock, a = a, inv.stock
	fmt.Println(keys(inv.stock), keys(a))
	c := a
	a, b = nil, a
	fmt.Println(a == nil, keys(b), keys(c), len(a))
}

