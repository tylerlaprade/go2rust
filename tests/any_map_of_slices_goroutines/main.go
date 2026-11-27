package main

import "fmt"

// A map of slices held in an interface prints as fmt does, a nil slice as
// [], wherever the interface is printed.

type P struct{ X int }

func show(v any) { fmt.Println(v) }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	m := map[string][]int{"b": {1, 2}, "a": nil}
	show(m)
	s := map[int][]string{2: {"x"}, 1: {}}
	fmt.Printf("%v|%d\n", s, len(s))
	ps := map[string][]P{"k": {{1}, {2}}}
	show(ps)
	fmt.Println(fmt.Sprint(m), []any{m, 3})
}
