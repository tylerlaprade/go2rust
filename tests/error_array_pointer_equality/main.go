package main

import "fmt"

type arrayPointerError struct {
	pointers [1]*int
	nested   [1][1]*int
}

func (arrayPointerError) Error() string { return "array pointer" }

func main() {
	left := 1
	right := 1

	var first error = arrayPointerError{pointers: [1]*int{&left}, nested: [1][1]*int{{&left}}}
	var same error = arrayPointerError{pointers: [1]*int{&left}, nested: [1][1]*int{{&left}}}
	var distinct error = arrayPointerError{pointers: [1]*int{&right}, nested: [1][1]*int{{&right}}}

	fmt.Println(first == same)
	fmt.Println(first == distinct)
}
