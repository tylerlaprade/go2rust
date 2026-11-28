package main

import "fmt"

func invoke(g func()) { g() }

type holder struct{ cb func() }

func main() {
	var f func()
	f = func() { f = nil; fmt.Println("ran") }
	invoke(f)
	fmt.Println(f == nil)
	h := &holder{}
	h.cb = func() { h.cb = nil; fmt.Println("cb") }
	invoke(h.cb)
	fmt.Println(h.cb == nil)
}
