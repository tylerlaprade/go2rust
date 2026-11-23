// An interface{} value is copied where Go copies it: a copy keeps its value
// after the source is reassigned, a parameter is the callee's own, and a
// closure that assigns a captured interface{} variable changes the variable.
package main

import "fmt"

type point struct{ X, Y int }

func get() any { return 7 }

func reassign(v any) any {
	v = "changed in callee"
	return v
}

func main() {
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
}
