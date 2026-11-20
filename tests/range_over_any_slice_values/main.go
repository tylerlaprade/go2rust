// The value of a range over a []interface{} is a reference to the element's
// box. Passing it to a func(interface{}), storing it in another
// interface{} and printing the default case of a type switch on it did
// not compile.
package main

import "fmt"

func show(v interface{}) { fmt.Println(v) }

func main() {
	values := []interface{}{"a", 1, 2.5, true}
	for _, val := range values {
		show(val)
	}
	for i, val := range values {
		var copy any = val
		fmt.Println(i, copy)
	}
	for _, val := range values {
		switch v := val.(type) {
		case string:
			fmt.Println("s", v)
		default:
			fmt.Printf("%T %v\n", v, v)
			show(v)
		}
	}
}
