// Go's int is 64 bits. It was generated as i32, so a value past
// 2,147,483,647 wrapped: fib(90), 20!, 1 << 40 and math.MaxInt printed the
// wrong numbers.
package main

import (
	"fmt"
	"math"
	"strconv"
)

var memo = map[int]int{}

func fib(n int) int {
	if n < 2 {
		return n
	}
	if v, ok := memo[n]; ok {
		return v
	}
	v := fib(n-1) + fib(n-2)
	memo[n] = v
	return v
}

func factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorial(n-1)
}

func main() {
	fmt.Println(fib(90), factorial(20))
	big := 1 << 40
	fmt.Println(big, big*3, math.MaxInt, math.MinInt)
	var total int
	for i := 0; i < 100000; i++ {
		total += i * i
	}
	fmt.Println(total)
	n, _ := strconv.Atoi("9000000000")
	fmt.Println(n+1, strconv.Itoa(n*2))
	wrapped := math.MaxInt
	wrapped++
	fmt.Println(wrapped == math.MinInt)
	var x interface{} = 7
	_, isInt := x.(int)
	fmt.Printf("%T %v %d\n", x, isInt, len("hello")*1000000000)
	var r rune = 'A'
	fmt.Printf("%T %v %c\n", r, r, r)
}
