package main

import "fmt"

func makeCounter() func() int {
	count := 0
	return func() int {
		count++
		return count
	}
}

func makeAdder(x int) func(int) int {
	return func(y int) int {
		return x + y
	}
}

func applyOperation(nums []int, op func(int) int) []int {
	result := make([]int, len(nums))
	for i, num := range nums {
		result[i] = op(num)
	}
	return result
}

func main() {
	// Basic closure
	counter := makeCounter()
	fmt.Println("Counter 1:", counter())
	fmt.Println("Counter 2:", counter())
	fmt.Println("Counter 3:", counter())

	// Another counter instance
	counter2 := makeCounter()
	fmt.Println("Counter2 1:", counter2())
	fmt.Println("Counter 4:", counter())

	// Closure with parameters
	add5 := makeAdder(5)
	add10 := makeAdder(10)

	fmt.Println("5 + 3 =", add5(3))
	fmt.Println("10 + 7 =", add10(7))

	// Higher-order functions
	numbers := []int{1, 2, 3, 4, 5}

	// Square function
	squared := applyOperation(numbers, func(x int) int {
		return x * x
	})
	fmt.Println("Squared:", squared)

	// Double function
	doubled := applyOperation(numbers, func(x int) int {
		return x * 2
	})
	fmt.Println("Doubled:", doubled)

	// Closure capturing local variable
	multiplier := 3
	tripled := applyOperation(numbers, func(x int) int {
		return x * multiplier
	})
	fmt.Println("Tripled:", tripled)

	// Immediately invoked function
	result := func(a, b int) int {
		return a + b
	}(10, 20)
	fmt.Println("Immediate result:", result)
}
