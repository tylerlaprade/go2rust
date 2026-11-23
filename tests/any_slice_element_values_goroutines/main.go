// The same program with a goroutine, so every value is behind a lock.
// Elements of a []any read as interface values of their own: printed, named
// with %T, asserted, compared, and kept after the slice changes.
package main

import "fmt"

type box struct {
	items []any
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	mixed := []any{1, "two", 3.5, true, 7}
	v := mixed[0]
	fmt.Println(v)
	fmt.Println(mixed[1])
	fmt.Printf("%v %v\n", mixed[2], mixed[3])
	s := mixed[1].(string)
	fmt.Println(s + "!")
	fmt.Println(mixed[4] == nil, mixed[0] == 1, mixed[4] == 7)
	b := box{items: mixed}
	fmt.Println(b.items[1], len(b.items))
	mixed[0] = "changed"
	fmt.Println(mixed[0], v)
	fmt.Println(fmt.Sprint(mixed[3]), fmt.Sprintf("%T", mixed[2]))
}
