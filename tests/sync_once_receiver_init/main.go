package main

import (
	"fmt"
	"sync"
)

type runner struct {
	once       sync.Once
	inFlight   chan struct{}
	serialized chan struct{}
}

func (r *runner) initialize() {
	r.once.Do(func() {
		r.inFlight = make(chan struct{}, 2)
		r.serialized = make(chan struct{}, 1)
	})
}

func main() {
	r := &runner{}
	r.initialize()
	r.initialize()

	inFlightCap := cap(r.inFlight)
	serializedCap := cap(r.serialized)
	fmt.Println(inFlightCap, serializedCap)
}
