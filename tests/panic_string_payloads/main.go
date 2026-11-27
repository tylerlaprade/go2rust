// A string passed to panic keeps its text, even one that reads like the
// message Rust gives one of its own runtime errors, while a runtime error
// still gets Go's text. Calling main again keeps the panic handling the
// first call set up, so the second call's recover still stops its panic.
package main

import "fmt"

var calls int

func recovered(f func()) {
	defer func() { fmt.Println("recovered:", recover()) }()
	f()
}

func main() {
	calls++
	if calls == 2 {
		defer func() { fmt.Println("second main recovered:", recover()) }()
		panic("from the second main")
	}
	recovered(func() { panic("attempt to divide by zero") })
	zero := 0
	recovered(func() { fmt.Println(1 / zero) })
	recovered(func() { panic("called `Option::unwrap()` on a `None` value") })
	recovered(func() { panic(fmt.Sprintf("attempt to %s by zero", "divide")) })
	main()
	fmt.Println("back in the first main")
	panic("attempt to calculate the remainder with a divisor of zero")
}
