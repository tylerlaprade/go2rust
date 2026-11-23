// Printing a map held in a struct field, through a pointer, a value, an
// embedded struct and a nested field.
package main

import (
	"fmt"
	"sync"
)

type counts struct {
	byName map[string]int
}

type Inventory struct {
	mu sync.Mutex
	counts
	items map[string]int
	tags  map[int][]string
	inner struct{ m map[string]bool }
}

func (inv *Inventory) Add(name string, n int) {
	inv.mu.Lock()
	defer inv.mu.Unlock()
	inv.items[name] += n
}

func main() {
	inv := &Inventory{items: map[string]int{}, tags: map[int][]string{1: {"a", "b"}}}
	inv.byName = map[string]int{"z": 26}
	inv.inner.m = map[string]bool{"ok": true}
	inv.Add("apple", 2)
	inv.Add("pear", 1)
	inv.Add("apple", 3)
	fmt.Println(inv.items)
	fmt.Println(inv.byName, inv.tags)
	fmt.Println(inv.inner.m)
	fmt.Printf("%v %d\n", inv.items, len(inv.items))
	v := counts{byName: map[string]int{"b": 2, "a": 1}}
	fmt.Println(v.byName)
	var empty Inventory
	fmt.Println(empty.items, empty.items == nil)
}
