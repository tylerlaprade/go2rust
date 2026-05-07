package main

import "fmt"

func single(n int) []int {
	return []int{n}
}

func pair(a, b string) []string {
	return []string{a, b}
}

func main() {
	nums := single(4)
	words := pair("go", "rust")

	fmt.Println(len(nums), nums[0])
	fmt.Println(len(words), words[0], words[1])
}
