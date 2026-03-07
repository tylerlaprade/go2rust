package main

import "fmt"

func factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorial(n-1)
}

func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func power(base, exp int) int {
	if exp == 0 {
		return 1
	}
	if exp == 1 {
		return base
	}
	if exp%2 == 0 {
		half := power(base, exp/2)
		return half * half
	}
	return base * power(base, exp-1)
}

func sumArray(arr []int) int {
	if len(arr) == 0 {
		return 0
	}
	if len(arr) == 1 {
		return arr[0]
	}
	return arr[0] + sumArray(arr[1:])
}

func reverseString(s string) string {
	if len(s) <= 1 {
		return s
	}
	return reverseString(s[1:]) + string(s[0])
}

func main() {
	// Factorial
	fmt.Println("Factorial of 5:", factorial(5))
	fmt.Println("Factorial of 0:", factorial(0))

	// Fibonacci
	fmt.Println("Fibonacci sequence:")
	for i := 0; i < 10; i++ {
		fmt.Printf("fib(%d) = %d\n", i, fibonacci(i))
	}

	// GCD
	fmt.Println("GCD of 48 and 18:", gcd(48, 18))
	fmt.Println("GCD of 17 and 13:", gcd(17, 13))

	// Power
	fmt.Println("2^8 =", power(2, 8))
	fmt.Println("3^4 =", power(3, 4))
	fmt.Println("5^0 =", power(5, 0))

	// Sum array
	numbers := []int{1, 2, 3, 4, 5}
	fmt.Println("Sum of", numbers, "=", sumArray(numbers))

	// Reverse string
	original := "hello"
	reversed := reverseString(original)
	fmt.Printf("'%s' reversed is '%s'\n", original, reversed)
}
