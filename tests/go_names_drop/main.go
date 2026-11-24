// Go code may name a variable or a function drop. The generated code
// releases its own lock guards with Rust's drop, which such a name must not
// shadow.
package main

import (
	"fmt"
	"strings"
)

func drop(items []string, n int) []string {
	if n > len(items) {
		n = len(items)
	}
	return items[n:]
}

type queue struct {
	items []string
	seen  map[string]int
}

func main() {
	drop1 := 1
	words := strings.Fields("a b c d")
	rest := drop(words, drop1)
	for _, w := range rest {
		fmt.Print(w, " ")
	}
	fmt.Println()
	q := queue{items: words, seen: map[string]int{}}
	drop := 2
	for i, w := range q.items[drop:] {
		q.seen[w] = i + drop
	}
	keys := make([]string, 0)
	for k := range q.seen {
		keys = append(keys, k)
	}
	fmt.Println(len(keys), q.seen["c"], q.seen["d"], drop)
}
