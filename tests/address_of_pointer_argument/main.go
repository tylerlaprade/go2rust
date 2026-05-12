package main

import "fmt"

type info struct {
	name  string
	value int
}

type table struct {
	last *info
}

func accept(ptr *info) string {
	return ptr.name
}

func (t *table) register(name string, ptr *info) {
	t.last = ptr
	fmt.Println(name, ptr.value)
}

func main() {
	fmt.Println(accept(&info{name: "alpha", value: 7}))

	t := &table{}
	t.register("beta", &info{name: "beta", value: 9})
	fmt.Println(t.last.name)
}
