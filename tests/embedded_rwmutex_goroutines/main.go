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
	// shared.Unlock() through the same variable. The contention is best
	// effort: the sleep gives the workers time to reach Lock, and on a
	// transpiler that held the cell this run timed out.
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

	// A reader that waits behind a writer gets the lock before the next
	// writer does, as in Go. Each round gives the reader, then a second
	// writer, time to start waiting before the lock is let go; a goroutine
	// that starts late only changes that round's order. Over rounds with
	// longer waits, a lock that lets waiting readers in first shows it.
	readerFirst := false
	for round, wait := 0, 10*time.Millisecond; round < 5 && !readerFirst; round, wait = round+1, wait*2 {
		var mu sync.RWMutex
		mu.Lock()
		order := make(chan string, 2)
		finished := make(chan bool)
		go func() {
			mu.RLock()
			order <- "reader"
			mu.RUnlock()
			finished <- true
		}()
		time.Sleep(wait)
		go func() {
			mu.Lock()
			order <- "writer"
			mu.Unlock()
			finished <- true
		}()
		time.Sleep(wait)
		mu.Unlock()
		<-finished
		<-finished
		readerFirst = <-order == "reader"
	}
	fmt.Println("waiting reader before the next writer:", readerFirst)

	// Goroutines lock a mutex reached through pointer fields while main
	// holds it, and main unlocks it through the same fields: each waits
	// holding none of the structs on the way. Contention is best effort, as
	// above.
	deep := Outer{Middle: &Middle{Deep: &Deep{}}, Shallow: &Shallow{}}
	deep.Deep.Lock()
	started := make(chan bool)
	var wg4 sync.WaitGroup
	for i := 0; i < 2; i++ {
		wg4.Add(1)
		go func() {
			defer wg4.Done()
			started <- true
			deep.Deep.Lock()
			deep.Deep.Unlock()
		}()
	}
	<-started
	<-started
	time.Sleep(20 * time.Millisecond)
	deep.Deep.Unlock()
	wg4.Wait()
	fmt.Println("deep shared")

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

	// A go statement takes the mutex it unlocks when it is run, not when
	// the goroutine starts.
	gheld := &Table{m: map[string]int{}}
	greg := &Registry{Table: gheld}
	greg.Lock()
	go greg.Unlock()
	greg.Table = &Table{m: map[string]int{}}
	gheld.Lock() // waits for the goroutine to unlock gheld
	gheld.Unlock()
	fmt.Println("go unlock")

	// Goroutines lock through a call and an index while main holds the
	// mutex; each waits holding no cell main needs to unlock it.
	regs := []*Registry{{Table: &Table{m: map[string]int{}}}}
	get := func() *Registry { return regs[0] }
	get().Lock()
	ready := make(chan bool)
	var wg5 sync.WaitGroup
	for i := 0; i < 2; i++ {
		wg5.Add(1)
		go func(i int) {
			defer wg5.Done()
			ready <- true
			if i == 0 {
				get().Lock()
				get().Unlock()
			} else {
				regs[0].Lock()
				regs[0].Unlock()
			}
		}(i)
	}
	<-ready
	<-ready
	time.Sleep(20 * time.Millisecond)
	regs[0].Unlock()
	wg5.Wait()
	fmt.Println("call and index receivers")
}
