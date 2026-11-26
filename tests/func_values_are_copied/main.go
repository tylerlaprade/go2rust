// A func value is copied: assigning a func variable afterwards does not
// change a copy of it, and a closure that captured the variable sees the
// assignment.
package main

import "fmt"

type Box struct {
	fn func(int) int
}

type Op func(int) int

func (o Op) Twice(x int) int { return o(o(x)) }

var now = func() int { return 1 }

func keep(g func(int) int) func(int) int { return g }

func swapIn(f func() string) func() string {
	old := f
	f = func() string { return "new" }
	_ = f
	return old
}

func makeAdder(n int) func(int) int { return func(x int) int { return x + n } }

func deferred() {
	act := func(i int) { fmt.Println("act", i) }
	defer act(1)
	saved := act
	act = func(i int) { fmt.Println("replaced", i) }
	saved(2)
	act(3)
}

func main() {
	deferred()

	inc := func(x int) int { return x + 1 }
	var other func(int) int
	other = inc
	inc = func(x int) int { return x * 10 }
	fmt.Println("assigned:", other(1), inc(1))

	b := Box{fn: inc}
	inc = func(x int) int { return -x }
	fmt.Println("field:", b.fn(2), inc(2))

	kept := keep(inc)
	inc = func(x int) int { return x * x }
	fmt.Println("argument:", kept(3), inc(3))

	b2 := b
	b.fn = inc
	fmt.Println("struct copy:", b2.fn(4), b.fn(4))
	b3 := b
	b.fn = func(x int) int { return 1000 + x }
	fmt.Println("struct copy, literal:", b3.fn(4), b.fn(4))

	fns := []func(int) int{inc}
	inc = func(x int) int { return 100 + x }
	fmt.Println("slice:", fns[0](5), inc(5))

	m := map[string]func(int) int{"a": inc}
	inc = func(x int) int { return 7 }
	m["b"] = inc
	fmt.Println("map:", m["a"](1), m["b"](1))

	// A closure that calls a variable sees the variable's later value.
	var cb func() string
	run := func() string { return cb() }
	h := func() string { return "h" }
	cb = h
	fmt.Println("captured:", run())
	h = func() string { return "changed h" }
	fmt.Println("captured after:", run(), h())

	// Wrapping a function in a new one that calls the old one.
	log := func(s string) string { return "log:" + s }
	next := log
	log = func(s string) string { return "before " + next(s) }
	fmt.Println(log("x"))

	// Saving and restoring a package-level func variable.
	orig := now
	now = func() int { return 42 }
	fmt.Println("global:", now())
	now = orig
	fmt.Println("restored:", now())

	// Recursion through a variable, and a copy of it.
	var fib func(int) int
	fib = func(n int) int {
		if n < 2 {
			return n
		}
		return fib(n-1) + fib(n-2)
	}
	f2 := fib
	fmt.Println("fib:", fib(10), f2(11))

	// nil copies stay nil.
	var z func() string
	fmt.Println("nil:", z == nil)
	z = cb
	cb = nil
	fmt.Println("nil after:", z == nil, cb == nil, z())

	// A named func type with a method.
	var o Op = func(x int) int { return x + 1 }
	o2 := o
	o = func(x int) int { return x * 3 }
	fmt.Println("named:", o2.Twice(1), o.Twice(1))

	// A callee that assigns its parameter does not change the caller's.
	g := func() string { return "orig" }
	old := swapIn(g)
	fmt.Println("param:", old(), g())

	// A function's result.
	adder := makeAdder(5)
	savedAdder := adder
	adder = makeAdder(100)
	fmt.Println("result:", savedAdder(1), adder(1))

	// A closure that assigns a captured func variable.
	step := func() string { return "a" }
	advance := func() { step = func() string { return "b" } }
	before := step
	advance()
	fmt.Println("closure assigns:", before(), step())

	// A function that assigns the variable it was called through.
	var self func() string
	self = func() string {
		self = func() string { return "second" }
		return "first"
	}
	fmt.Println("self:", self(), self())
}
