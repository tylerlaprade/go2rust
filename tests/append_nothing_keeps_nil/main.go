// append(s, xs...) with no xs returns s: a nil slice stays nil, and an
// empty one stays empty and not nil.
package main

import "fmt"

func main() {
	var t []int
	t = append(t, []int{}...)
	fmt.Println(t == nil, len(t))
	var none []int
	t = append(t, none...)
	fmt.Println(t == nil)
	t = append(t, []int{1, 2}...)
	fmt.Println(t == nil, t)
	e := []int{}
	e = append(e, none...)
	fmt.Println(e == nil, len(e))
	var words []string
	more := []string{"a"}
	words = append(words, more[:0]...)
	fmt.Println(words == nil)
	words = append(words, more...)
	fmt.Println(words)
}
