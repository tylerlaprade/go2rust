package main

import "fmt"

// Assigning a slice expression into a value-typed slice element slot
// (a[i] = s[lo:hi:cap] where a is [][]byte). The element slot stores a raw Vec;
// the slice expression lowers to a wrapped handle, so it must be unwrapped to
// match the raw slot (E0308 otherwise). Mirrors bytes' genSplit/Fields.
func main() {
	s := []byte("hello world")
	a := make([][]byte, 2)
	a[0] = s[0:5:5]
	a[1] = s[6:11]
	fmt.Println(len(a[0]), cap(a[0]), len(a[1]))
}
