// In a program with goroutines: the characters of a string that is itself a
// range variable: each line of a grid, with and without indexes, and the
// first rune of each word.
package main

import (
	"fmt"
	"strings"
	"sync"
)

func main() {
	lines := []string{"a#", "#b", "é#"}
	n := 0
	for r, line := range lines {
		for c, ch := range line {
			if ch == '#' {
				n += r*10 + c
			}
		}
	}
	count := 0
	for _, line := range lines {
		for _, ch := range line {
			if ch != '#' {
				count++
			}
		}
		for i := range line {
			_ = i
		}
	}
	words := strings.Fields("go to rust")
	var initials []rune
	for _, w := range words {
		for _, ch := range w {
			initials = append(initials, ch)
			break
		}
	}
	fmt.Println(n, count, string(initials))
	var wg sync.WaitGroup
	hashes := make([]int, len(lines))
	for i, line := range lines {
		wg.Add(1)
		go func(i int, line string) {
			defer wg.Done()
			for _, ch := range line {
				if ch == '#' {
					hashes[i]++
				}
			}
		}(i, line)
	}
	wg.Wait()
	fmt.Println(hashes)
	s := "abc"
	for s := range s {
		fmt.Print(s, " ")
	}
	fmt.Println()
	for _, line := range []string{"ab", "cd"} {
		for _, ch := range line {
			line = ""
			fmt.Print(string(ch))
		}
		fmt.Print("[", line, "]")
	}
	fmt.Println()
	var counts []func() int
	for _, w := range []string{"xy", "zzz"} {
		counts = append(counts, func() int {
			n := 0
			for range w {
				n++
			}
			return n
		})
	}
	for _, f := range counts {
		fmt.Print(f(), " ")
	}
	fmt.Println()
}
