package main

import "fmt"

type Key interface {
	Name() string
}

type namedKey struct {
	name string
}

func (k namedKey) Name() string {
	return k.name
}

type Finder interface {
	Find(key Key) string
}

type finder struct{}

func (f finder) Find(key Key) string {
	return key.Name()
}

func main() {
	key := namedKey{name: "alpha"}
	finder := finder{}
	found := finder.Find(key)
	fmt.Println(found)
}
