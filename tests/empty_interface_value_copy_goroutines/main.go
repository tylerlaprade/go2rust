// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type point struct{ X, Y int }

func get() any { return 7 }

func reassign(v any) any {
	v = "changed in callee"
	return v
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	var a any = "first"
	f := func() { a = get() }
	f()
	fmt.Println(a)

	var b any = "x"
	c := b
	b = 9
	fmt.Println(b, c)
	g := func() { b = "from closure" }
	g()
	fmt.Println(b, c)

	var p any = point{1, 2}
	q := p
	p = point{3, 4}
	fmt.Println(p, q)

	var arg any = 1.5
	out := reassign(arg)
	fmt.Println(arg, out)

	holder := struct{ V any }{V: "held"}
	copied := holder.V
	holder.V = 42
	fmt.Println(holder.V, copied)

	var r any
	h := func() { r = point{5, 6} }
	h()
	fmt.Println(r, r == nil)

	var typedNil any = (*int)(nil)
	reassign(typedNil)
	fmt.Println(typedNil == nil, typedNil.(*int) == nil)
}
