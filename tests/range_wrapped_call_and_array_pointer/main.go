package main

import "fmt"

type bucket struct {
	values [3]int
}

func names() []string {
	return []string{"alpha", "beta"}
}

func (b *bucket) Sum() int {
	total := 0
	for _, value := range &b.values {
		total += value
	}
	return total
}

func main() {
	for _, name := range names() {
		fmt.Println(name)
	}
	b := &bucket{values: [3]int{2, 3, 5}}
	fmt.Println(b.Sum())

	groups := [2][]string{{"go"}, {"rust", "zig"}}
	total := 0
	for _, group := range &groups {
		for _, name := range group {
			if name != "" {
				total++
			}
		}
	}
	fmt.Println(total)
}
