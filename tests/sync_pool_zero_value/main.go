// A sync.Pool without New: Get gives nil while the pool is empty, and a
// program that never names New still compiles.
package main

import (
	"fmt"
	"sync"
)

type machine struct{ steps int }

var machines sync.Pool

func get() *machine {
	m, ok := machines.Get().(*machine)
	if !ok {
		m = new(machine)
	}
	return m
}

func main() {
	m := get()
	m.steps = 3
	fmt.Println(m.steps, machines.Get() == nil)
	machines.Put(m)
}
