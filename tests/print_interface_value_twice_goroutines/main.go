package main

import "fmt"

// describe prints its argument more than once in each call. In a program
// with a goroutine every variable is locked while it is read, and reading
// the same one twice in a statement must not wait on itself.
func describe(v interface{}) {
	fmt.Printf("%v|%v\n", v, v)
	fmt.Printf("%T: %v\n", v, v)
	fmt.Println(v, v)
	text := fmt.Sprintf("%v and %v", v, v)
	fmt.Println(text)
}

type Holder struct{ value interface{} }

func main() {
	// A goroutine makes every shared value a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	describe(5)
	describe("text")
	describe(true)

	var held interface{} = 2.5
	fmt.Printf("%v %v %v\n", held, held, held)
	h := Holder{value: true}
	fmt.Println(h.value, h.value)
}
