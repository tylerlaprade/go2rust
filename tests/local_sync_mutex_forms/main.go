// Every way of declaring a local sync.Mutex or sync.RWMutex, in a program
// without goroutines. "var mu sync.Mutex" followed by mu.Lock() did not
// compile, "var rw sync.RWMutex" had no initializer, and "&sync.Mutex{}"
// could not be passed to a *sync.Mutex parameter.
package main

import (
	"fmt"
	"sync"
)

var global sync.Mutex

func bump(mu *sync.Mutex, n *int) {
	mu.Lock()
	*n = *n + 1
	mu.Unlock()
}

func read(rw *sync.RWMutex, n *int) int {
	rw.RLock()
	defer rw.RUnlock()
	return *n
}

func locked(mu *sync.Mutex) int {
	mu.Lock()
	defer mu.Unlock()
	return 7
}

func main() {
	n := 0
	a := sync.Mutex{}
	a.Lock()
	n++
	a.Unlock()

	var b = sync.Mutex{}
	b.Lock()
	n++
	b.Unlock()

	c := &sync.Mutex{}
	bump(c, &n)

	var d sync.Mutex
	bump(&d, &n)
	fmt.Println(n, locked(&d), locked(c))

	global.Lock()
	n++
	global.Unlock()

	var rw sync.RWMutex
	rw.RLock()
	rw.RLock()
	fmt.Println(read(&rw, &n))
	rw.RUnlock()
	rw.RUnlock()
	rw.Lock()
	n = 10
	rw.Unlock()
	fmt.Println(read(&rw, &n))
}
