package main

import (
	"fmt"
	"sort"
	"sync"
	"time"
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

func main() {
	t := &Table{m: map[string]int{"a": 1}}
	var wg sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			t.Write(fmt.Sprintf("w%d", i), i)
			_ = t.Read("a")
		}(i)
	}
	wg.Wait()
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

	// Two Registry values share one Table: a goroutine waiting for the
	// mutex through one holds nothing the owner needs to unlock it.
	r2 := Registry{Table: t, name: "reg2"}
	var wg2 sync.WaitGroup
	for i := 0; i < 8; i++ {
		wg2.Add(1)
		go func(i int) {
			defer wg2.Done()
			reg := r
			if i%2 == 1 {
				reg = r2
			}
			reg.Lock()
			reg.m[fmt.Sprintf("s%d", i)] = i
			reg.Unlock()
		}(i)
	}
	wg2.Wait()
	fmt.Println(len(t.m))

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

	// Goroutines lock a Registry variable they share while main holds the
	// lock. Each waits for the mutex holding nothing main needs to reach
	// shared.Unlock() through the same variable.
	shared := Registry{Table: &Table{m: map[string]int{}}, name: "shared"}
	shared.Lock()
	waiting := make(chan bool)
	var wg3 sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg3.Add(1)
		go func(i int) {
			defer wg3.Done()
			waiting <- true
			shared.Lock()
			shared.m[fmt.Sprint("k", i)] = i
			shared.Unlock()
			shared.RLock()
			_ = len(shared.m)
			shared.RUnlock()
		}(i)
	}
	for i := 0; i < 4; i++ {
		<-waiting
	}
	time.Sleep(20 * time.Millisecond)
	shared.Unlock()
	wg3.Wait()
	fmt.Println(shared.name, len(shared.m))
}
