// An LRU cache on container/list, translated from source: MoveToFront and
// Remove find the element in its list by comparing e.list with the list.
package main

import (
	"container/list"
	"errors"
	"fmt"
	"sort"
	"strings"
)

type entry struct {
	key   string
	value int
}

type LRU struct {
	cap   int
	ll    *list.List
	items map[string]*list.Element
}

func NewLRU(capacity int) *LRU {
	return &LRU{cap: capacity, ll: list.New(), items: make(map[string]*list.Element)}
}

var ErrMissing = errors.New("missing key")

func (c *LRU) Get(key string) (int, error) {
	if el, ok := c.items[key]; ok {
		c.ll.MoveToFront(el)
		return el.Value.(*entry).value, nil
	}
	return 0, fmt.Errorf("get %q: %w", key, ErrMissing)
}

func (c *LRU) Put(key string, value int) {
	if el, ok := c.items[key]; ok {
		el.Value.(*entry).value = value
		c.ll.MoveToFront(el)
		return
	}
	el := c.ll.PushFront(&entry{key, value})
	c.items[key] = el
	if c.ll.Len() > c.cap {
		last := c.ll.Back()
		c.ll.Remove(last)
		delete(c.items, last.Value.(*entry).key)
	}
}

func (c *LRU) Keys() []string {
	var keys []string
	for el := c.ll.Front(); el != nil; el = el.Next() {
		keys = append(keys, el.Value.(*entry).key)
	}
	return keys
}

func main() {
	c := NewLRU(2)
	c.Put("a", 1)
	c.Put("b", 2)
	v, _ := c.Get("a")
	fmt.Println(v, c.Keys())
	c.Put("c", 3)
	_, err := c.Get("b")
	fmt.Println(err, errors.Is(err, ErrMissing))
	c.Put("a", 10)
	fmt.Println(c.Keys(), len(c.items))
	words := strings.Fields("b c a b a a")
	freq := map[string]int{}
	for _, w := range words {
		freq[w]++
	}
	type kv struct {
		k string
		n int
	}
	var kvs []kv
	for k, n := range freq {
		kvs = append(kvs, kv{k, n})
	}
	sort.Slice(kvs, func(i, j int) bool {
		if kvs[i].n != kvs[j].n {
			return kvs[i].n > kvs[j].n
		}
		return kvs[i].k < kvs[j].k
	})
	fmt.Println(kvs)
	var sb strings.Builder
	for i, e := range kvs {
		if i > 0 {
			sb.WriteString(", ")
		}
		fmt.Fprintf(&sb, "%s=%d", e.k, e.n)
	}
	fmt.Println(sb.String())
}
