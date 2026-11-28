package main

import "fmt"

type timer struct{ fn func() }

type ctx struct {
	t     *timer
	count int
	hook  func() int
}

func after(f func()) *timer { return &timer{fn: f} }

func main() {
	c := &ctx{}
	c.t = after(func() { c.count++ })
	c.hook = func() int { return c.count * 10 }
	c.t.fn()
	c.t.fn()
	fmt.Println(c.count, c.hook())
	m := map[string]func() int{}
	n := 3
	m["k"] = func() int { return n }
	n = 4
	fmt.Println(m["k"]())
}
