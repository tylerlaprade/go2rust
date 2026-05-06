package main

import "fmt"

type Branch struct {
	Else int
}

func useKeywordNames(fn int) int {
	total := 0
	for _, mod := range []int{fn, 2} {
		total += mod
	}
	branch := Branch{Else: total}
	return branch.Else
}

func main() {
	fmt.Println(useKeywordNames(3))
}
