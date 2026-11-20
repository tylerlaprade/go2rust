// A failed one-value type assertion panics with Go's message, which names
// the dynamic type and the asserted one. It said "type assertion failed".
// Each assertion reads its own variable: in a program with goroutines a
// panic while the value is locked still poisons that lock.
package main

import "fmt"

type Rect struct{ W, H int }

func try(f func()) {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("recovered:", r)
		}
	}()
	f()
}

func main() {
	var a interface{} = 42
	try(func() { _ = a.(string) })
	var b interface{} = 42
	try(func() { _ = b.(Rect) })
	var c interface{} = 42
	try(func() { _ = c.(*Rect) })
	var d interface{} = Rect{1, 2}
	try(func() { _ = d.(int) })
	var e interface{} = Rect{1, 2}
	try(func() { fmt.Println(e.(Rect).W) })
	var f interface{}
	try(func() { _ = f.(int) })
	items := []interface{}{"a", 2.5}
	try(func() { _ = items[1].(string) })
	var g interface{} = "text"
	try(func() { _ = g.([]int) })
	var h interface{} = 3.5
	try(func() { fmt.Println(h.(float64)) })
}
