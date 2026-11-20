// An error taken out of an interface{} is a copy, like every other use of an
// error value: assigning to the result must not change what the interface
// value holds. The one-value form "x.(error)" must also find an error made by
// errors.New; it used to stop the program.
package main

import (
	"errors"
	"fmt"
)

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var boxed any = errors.New("x")

	got, ok := boxed.(error)
	fmt.Println(ok, got)
	got = nil
	again, ok := boxed.(error)
	fmt.Println(ok, again, got)

	single := boxed.(error)
	fmt.Println(single)
	single = errors.New("replaced")
	third, ok := boxed.(error)
	fmt.Println(ok, third, single)

	switch v := boxed.(type) {
	case error:
		fmt.Println("case error:", v)
		v = nil
		fmt.Println(v == nil)
	}
	fourth, ok := boxed.(error)
	fmt.Println(ok, fourth)

	// The copy is still the same error.
	sentinel := errors.New("sentinel")
	var held any = sentinel
	back := held.(error)
	fmt.Println(back == sentinel, back)

	// A formatted error takes the same paths.
	var wrapped any = fmt.Errorf("wrapped: %d", 7)
	w := wrapped.(error)
	w2, ok := wrapped.(error)
	fmt.Println(w, w2, ok)
	w = nil
	w3 := wrapped.(error)
	fmt.Println(w == nil, w3)
}
