// In a program with goroutines: generic functions over []byte | string in
// another package keep their generic code.
package main

import (
	"fmt"
	"sync"

	"example.com/seqpkggo/seq"
)

func main() {
	fmt.Println(seq.Trim("hello", 2), string(seq.Trim([]byte("hello"), 3)))
	fmt.Println(seq.Mid("abcdef", 1, 4), string(seq.Mid([]byte("abcdef"), 2, 5)))
	fmt.Println(seq.Digits("a1b22c"), seq.Digits([]byte("9x9")))
	fmt.Println(seq.Prefix("xyz"), seq.Prefix([]byte("pq!")))
	n, rest := seq.Atoi("123abc")
	m, restb := seq.Atoi([]byte("42!"))
	fmt.Println(n, rest, m, string(restb))
	b := []byte("shared")
	t := seq.Trim(b, 1)
	t[0] = 'H'
	fmt.Println(string(b), string(t))
	var wg sync.WaitGroup
	counts := make([]int, 2)
	for i, s := range []string{"a1", "22b3"} {
		wg.Add(1)
		go func(i int, s string) {
			defer wg.Done()
			counts[i] = seq.Digits(s)
		}(i, s)
		wg.Wait()
	}
	fmt.Println(counts)
}
