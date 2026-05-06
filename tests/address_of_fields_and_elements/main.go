package main

import "fmt"

type Point struct{ X int }

func main() {
	p := Point{X: 10}
	px := &p.X
	*px = 20

	nums := []int{1, 2, 3}
	first := &nums[0]
	*first = 9

	fmt.Println(p.X, nums)
}
