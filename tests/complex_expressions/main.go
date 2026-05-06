package main

import "fmt"

func main() {
	// Complex arithmetic expressions
	fmt.Println("=== Complex arithmetic expressions ===")

	a, b, c := 10, 20, 30

	// Nested arithmetic
	result1 := (a+b)*c - (a*b)/(c-a)
	fmt.Printf("(a + b) * c - (a * b) / (c - a) = %d\n", result1)

	// Mixed operations with precedence
	result2 := a + b*c/(a-5) + c%b
	fmt.Printf("a + b * c / (a - 5) + c %% b = %d\n", result2)

	// Boolean expressions
	fmt.Println("\n=== Complex boolean expressions ===")

	x, y, z := 5, 10, 15

	// Complex boolean logic
	bool1 := (x < y) && (y < z) || (x == 5 && z > 10)
	fmt.Printf("(x < y) && (y < z) || (x == 5 && z > 10) = %t\n", bool1)

	bool2 := !(x > y) && (z-y == x) || (x*2 == y)
	fmt.Printf("!(x > y) && (z-y == x) || (x*2 == y) = %t\n", bool2)

	// Bitwise operations
	fmt.Println("\n=== Complex bitwise expressions ===")

	bits1, bits2 := 0b1010, 0b1100

	bitwiseResult := (bits1 & bits2) | (bits1^bits2)<<1
	fmt.Printf("(bits1 & bits2) | (bits1 ^ bits2) << 1 = %b (%d)\n", bitwiseResult, bitwiseResult)

	// Function calls in expressions
	fmt.Println("\n=== Function calls in expressions ===")

	getValue := func(n int) int { return n * 2 }
	getMultiplier := func() int { return 3 }

	complexResult := getValue(a) + getValue(b)*getMultiplier() - getValue(c)/2
	fmt.Printf("getValue(a) + getValue(b) * getMultiplier() - getValue(c)/2 = %d\n", complexResult)

	// Array/slice expressions
	fmt.Println("\n=== Array/slice expressions ===")

	numbers := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

	// Complex indexing
	idx1, idx2 := 2, 7
	sliceResult := numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0]
	fmt.Printf("numbers[idx1:idx2][1] + numbers[len(numbers)-1] - numbers[0] = %d\n", sliceResult)

	// Map expressions
	fmt.Println("\n=== Map expressions ===")

	data := map[string]int{
		"alpha": 10,
		"beta":  20,
		"gamma": 30,
	}

	mapResult := data["alpha"] + data["beta"]*2 - data["gamma"]/3
	fmt.Printf("data[\"alpha\"] + data[\"beta\"]*2 - data[\"gamma\"]/3 = %d\n", mapResult)

	// Struct field expressions
	fmt.Println("\n=== Struct field expressions ===")

	type Point struct {
		X, Y int
	}

	p1 := Point{X: 3, Y: 4}
	p2 := Point{X: 6, Y: 8}

	// Distance calculation (without sqrt for simplicity)
	distanceSquared := (p2.X-p1.X)*(p2.X-p1.X) + (p2.Y-p1.Y)*(p2.Y-p1.Y)
	fmt.Printf("Distance squared between points: %d\n", distanceSquared)

	// Pointer expressions
	fmt.Println("\n=== Pointer expressions ===")

	val := 42
	ptr := &val

	ptrResult := *ptr + (*ptr * 2) - (*ptr / 2)
	fmt.Printf("*ptr + (*ptr * 2) - (*ptr / 2) = %d\n", ptrResult)

	// Type assertion expressions
	fmt.Println("\n=== Type assertion expressions ===")

	var iface interface{} = 100

	if intVal, ok := iface.(int); ok {
		assertResult := intVal*2 + (intVal/5)*3
		fmt.Printf("Type assertion result: %d\n", assertResult)
	}

	// Channel expressions (non-blocking)
	fmt.Println("\n=== Channel expressions ===")

	ch := make(chan int, 3)
	ch <- 10
	ch <- 20
	ch <- 30

	chanResult := <-ch + <-ch*2 - <-ch/2
	fmt.Printf("Channel expression result: %d\n", chanResult)

	// Nested function calls
	fmt.Println("\n=== Nested function calls ===")

	add := func(a, b int) int { return a + b }
	multiply := func(a, b int) int { return a * b }
	subtract := func(a, b int) int { return a - b }

	nestedResult := add(multiply(3, 4), subtract(20, multiply(2, 5)))
	fmt.Printf("add(multiply(3, 4), subtract(20, multiply(2, 5))) = %d\n", nestedResult)

	// Complex conditional expressions
	fmt.Println("\n=== Complex conditional expressions ===")

	score := 85
	var grade string

	// Ternary-like using if-else
	if score >= 90 {
		grade = "A"
	} else if score >= 80 {
		grade = "B"
	} else if score >= 70 {
		grade = "C"
	} else {
		grade = "F"
	}

	fmt.Printf("Grade for score %d: %s\n", score, grade)

	// Complex assignment expressions
	fmt.Println("\n=== Complex assignment expressions ===")

	counter := 0
	counter += (5 * 3) - (10 / 2) + (8 % 3)
	fmt.Printf("Complex assignment result: %d\n", counter)

	// Multiple assignment with expressions
	sum, product := a+b+c, a*b*c
	fmt.Printf("Sum: %d, Product: %d\n", sum, product)

	// Range expressions
	fmt.Println("\n=== Range expressions ===")

	total := 0
	for i, val := range numbers[:5] {
		total += i*val + (val % 3)
	}
	fmt.Printf("Complex range calculation: %d\n", total)
}
