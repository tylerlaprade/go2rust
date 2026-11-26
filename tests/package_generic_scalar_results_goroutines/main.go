// A generic function of another package returns a type parameter in a
// cell, so a call that gives an int, a float or a bool is unwrapped like any
// other wrapped result.
package main

import (
	"fmt"

	"example.com/packagegenericscalarresultsgoroutines/dep"
)

func double(n int) int { return n * 2 }

func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	x := dep.Pick(7)
	y := dep.Pick[int](8) + 1
	var z int = dep.Pick(10)
	fmt.Println(x, y, z, x+z, dep.Pick(9))
	f := dep.Pick(1.5) * 2
	ok := dep.Pick(true) && !dep.Pick(false)
	fmt.Println(f, ok, double(dep.Pick(4)), dep.Size("s")+1)
	a, b := dep.Pair(3, 2.5)
	fmt.Println(a+1, b*2)
	a, b = dep.Pair(5, 0.5)
	fmt.Println(a, b)
	z = dep.Pick(11)
	z += dep.Max(2, 12)
	fmt.Println(z, dep.Max(1.5, -2.0), dep.Larger(3, 2))
	if dep.Larger(2.5, 1.0) && dep.Max(3, 4) == 4 {
		fmt.Println("compared")
	}
	nums := []int{dep.Pick(1), dep.Max(2, 1)}
	m := map[string]float64{"k": dep.Max(0.5, 0.25)}
	fmt.Println(nums, m, len(nums)+dep.Pick(0))
}
