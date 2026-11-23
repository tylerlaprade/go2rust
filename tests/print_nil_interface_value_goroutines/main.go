// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

func noPanic() (r any) {
	defer func() { fmt.Println("recover in defer:", recover()) }()
	return "unchanged"
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
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
