// A rune from ranging over a string is the key of a map[rune]V: counted,
// read, tested, written, deleted, and read with comma-ok.
package main

import (
	"fmt"
	"sort"
)

func main() {
	freq := map[rune]int{}
	seen := map[rune]bool{}
	firstAt := map[rune]int{}
	for i, r := range "hello, wörld" {
		freq[r]++
		if !seen[r] {
			seen[r] = true
			firstAt[r] = i
		}
	}
	delete(freq, ',')
	_, hasSpace := freq[' ']
	fmt.Println(len(freq), freq['l'], freq['ö'], hasSpace, firstAt['w'])
	for _, r := range "lo" {
		freq[r] += 10
		if n, ok := freq[r]; ok {
			fmt.Printf("%c=%d ", r, n)
		}
	}
	fmt.Println()
	keys := make([]rune, 0, len(freq))
	for r := range freq {
		keys = append(keys, r)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
	for _, r := range []rune("世界") {
		fmt.Printf("%c", r)
	}
	fmt.Printf(" %c%c\n", rune(0x4e16), -1)
	for _, r := range keys {
		fmt.Printf("%c:%d ", r, freq[r])
	}
	fmt.Println()
}
