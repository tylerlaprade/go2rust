package main

import (
	"fmt"
	"sort"
	"sync"
)

// A struct that embeds sync.RWMutex locks through its promoted methods, from
// its own methods, from outside, and in a defer.

type Table struct {
	sync.RWMutex
	m map[string]int
}

func (t *Table) Read(k string) int {
	t.RLock()
	defer t.RUnlock()
	return t.m[k]
}

func (t *Table) Write(k string, v int) {
	t.Lock()
	t.m[k] = v
	t.Unlock()
}

type Registry struct {
	*Table
	name string
}

// Swap replaces r's Table while holding the old one's lock: the deferred
// Unlock unlocks the Table that was locked, as Go follows r.Table to the
// mutex when the defer runs.
func (r *Registry) Swap() {
	r.Lock()
	defer r.Unlock()
	r.Table = &Table{m: map[string]int{"swapped": 1}}
}

// Bump locks through Table's mutex from a method of Registry.
func (r *Registry) Bump(k string) {
	r.Lock()
	r.m[k]++
	r.Unlock()
}

// Both embeds Table, which has its own mutex, and a mutex of its own: Go
// picks the shallower one, Both's own.
type Both struct {
	*Table
	sync.RWMutex
}

// Deep's mutex is two levels below Outer and Shallow's is one, so Go
// promotes Outer.Lock from Shallow.
type Deep struct {
	sync.RWMutex
}

type Middle struct {
	*Deep
}

type Shallow struct {
	sync.RWMutex
	n int
}

type Outer struct {
	*Middle
	*Shallow
}

// Picky's own mutex is one level below it and Custom's Lock two, so Go
// promotes Picky.Lock from the mutex, though *Deeper comes first.
type Custom struct{ calls int }

func (c *Custom) Lock()   { c.calls++ }
func (c *Custom) Unlock() { c.calls-- }

type Deeper struct {
	*Custom
}

type Picky struct {
	*Deeper
	sync.RWMutex
}

// Hold locks Picky's own mutex from a method of Picky.
func (p *Picky) Hold() {
	p.Lock()
	p.RWMutex.Unlock()
}

func main() {
	t := &Table{m: map[string]int{"a": 1}}
	t.Write("b", 2)
	t.Lock()
	t.m["c"] = 3
	t.Unlock()
	t.RLock()
	keys := []string{}
	for k := range t.m {
		keys = append(keys, k)
	}
	t.RUnlock()
	sort.Strings(keys)
	fmt.Println(keys, t.Read("b"), t.Read("z"))
	r := Registry{Table: t, name: "reg"}
	r.Write("d", 4)
	r.RLock()
	fmt.Println(r.name, len(r.m))
	r.RUnlock()

	r.Bump("d")
	fmt.Println(r.m["d"])
	both := &Both{Table: t}
	both.Lock()
	t.Lock() // Both's own mutex is held, not Table's
	t.Unlock()
	both.Unlock()
	both.RLock()
	fmt.Println("both", len(both.m))
	both.RUnlock()

	o := Outer{Middle: &Middle{Deep: &Deep{}}, Shallow: &Shallow{n: 7}}
	o.Lock()
	o.Shallow.Unlock() // the lock o.Lock took
	o.Deep.Lock()      // free: o.Lock did not take it
	o.Deep.Unlock()
	o.RLock()
	fmt.Println("outer", o.n)
	o.RUnlock()

	p := &Picky{Deeper: &Deeper{Custom: &Custom{}}}
	p.Hold()
	p.Lock()
	fmt.Println("picky locked", p.calls) // Custom.Lock did not run
	p.Unlock()
	fmt.Println("picky", p.calls)

	// A method value keeps the mutex it was made from when a field on the
	// way to it is reassigned.
	held := &Table{m: map[string]int{}}
	reg := &Registry{Table: held}
	held.Lock()
	unlock := reg.Unlock
	reg.Table = &Table{m: map[string]int{"x": 1}}
	unlock()
	held.Lock() // free again: unlock let go of held's mutex
	held.Unlock()
	fmt.Println("method value", len(reg.m))

	before := &Table{m: map[string]int{}}
	swapped := &Registry{Table: before}
	swapped.Swap()
	before.Lock() // free again: the deferred Unlock let go of it
	before.Unlock()
	swapped.Lock()
	swapped.Unlock()
	fmt.Println("deferred unlock", len(swapped.m))
}
