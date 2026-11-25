// A method written for a nil receiver, declared in one file of a package and
// called on a nil pointer from another file of it, as time's
// (*Location).get is called from time.go.
package main

import (
	"fmt"

	"example.com/nilrecv/list"
)

func main() {
	var empty *list.List
	fmt.Println(empty.Len(), list.Sum(empty))
	l := list.Push(list.Push(nil, 2), 3)
	fmt.Println(l.Len(), list.Sum(l))
}
