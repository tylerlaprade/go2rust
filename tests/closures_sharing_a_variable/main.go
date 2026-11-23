// Two or more function literals of one statement capture the same variable:
// a counter and its reset, named results, slice, map and struct literals of
// closures, and an append of two closures.
package main

import "fmt"

func counter() (func() int, func()) {
	c := 0
	return func() int { c++; return c }, func() { c = 0 }
}

func pair() (inc func(), get func() int) {
	n := 10
	inc = func() { n++ }
	get = func() int { return n }
	return
}

func main() {
	next, reset := counter()
	next()
	next()
	fmt.Println(next())
	reset()
	fmt.Println(next())
	inc, get := pair()
	inc()
	fmt.Println(get())
	ops := []func(int) int{
		func(x int) int { return x + 1 },
		func(x int) int { return x * 2 },
	}
	total := 0
	adders := map[string]func(){
		"one": func() { total += 1 },
		"ten": func() { total += 10 },
	}
	adders["one"]()
	adders["ten"]()
	fmt.Println(ops[0](3), ops[1](3), total)
}

type handlers struct {
	onAdd   func(int)
	onTotal func() int
}

func makeHandlers() handlers {
	sum := 0
	h := handlers{onAdd: func(n int) { sum += n }, onTotal: func() int { return sum }}
	return h
}

func init() {
	h := makeHandlers()
	h.onAdd(4)
	h.onAdd(5)
	fmt.Println("handlers:", h.onTotal())
	count := 0
	var steps []func()
	steps = append(steps, func() { count++ }, func() { count += 100 })
	for _, s := range steps {
		s()
	}
	fmt.Println("steps:", count)
}
