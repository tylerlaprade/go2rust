package main

import "fmt"

// Assigning a slice expression into a nested slice element slot
// (a[i] = s[lo:hi:cap] where a is [][]byte). The element slot retains the
// wrapped slice handle so later element updates share its backing storage.
func main() {
	s := []byte("hello world")
	a := make([][]byte, 2)
	a[0] = s[0:5:5]
	a[1] = s[6:11]
	fmt.Println(len(a[0]), cap(a[0]), len(a[1]))
}
