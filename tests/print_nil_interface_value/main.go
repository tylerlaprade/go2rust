// Printing an interface value that holds nothing writes <nil>, with %v and
// %T alike, including the nil that recover returns when nothing panicked.
package main

import "fmt"

func noPanic() (r any) {
	defer func() { fmt.Println("recover in defer:", recover()) }()
	return "unchanged"
}

func main() {
	var a any
	fmt.Println("a is", a)
	fmt.Printf("%v|%T|%d\n", a, a, 1)
	r := noPanic()
	fmt.Println("returned", r, r == nil)
	a = 42
	fmt.Println("a is", a)
	fmt.Printf("%v|%T\n", a, a)
	var e error
	fmt.Println("e is", e)
	defer func() {
		fmt.Println("deferred recover:", recover())
	}()
}
