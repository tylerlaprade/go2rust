package main

type Mutex struct {
	Value int
}

func newMutex() Mutex {
	return Mutex{Value: 7}
}
