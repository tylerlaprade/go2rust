// A function value taken out of an interface{}, by every form of type
// assertion. Only a direct call, x.(func())(), compiled: g := x.(func()),
// the comma-ok form, f = x.(func()) and an element of a []any all
// downcast to the bare closure box, which is not what the interface holds.
package main

import "fmt"

func run(f func()) { f() }

func main() {
	count := 0
	var x any = func() { count++ }
	g := x.(func())
	g()
	_, ok := x.(func(int))
	fmt.Println(count, ok)
	h, ok := x.(func())
	h()
	fmt.Println(count, ok)
	run(x.(func()))
	x.(func())()
	var k func()
	k = x.(func())
	k()
	fmt.Println(count)

	handlers := map[string]any{"inc": func(n int) int { return n + 1 }}
	inc := handlers["inc"].(func(int) int)
	fmt.Println(inc(41))

	items := []any{func() string { return "hi" }}
	say := items[0].(func() string)
	fmt.Println(say())
	say2, ok := items[0].(func() string)
	fmt.Println(say2(), ok)
	_, ok = items[0].(func())
	fmt.Println(ok)

	switch f := x.(type) {
	case func():
		f()
		fmt.Println("func", count)
	}
}
