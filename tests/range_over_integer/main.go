package main

import "fmt"

func main() {
	literalSum := 0
	for i := range 5 {
		literalSum += i
	}

	count := 0
	for range 3 {
		count++
	}

	n := 4
	variableSum := 0
	for i := range n {
		variableSum += i
	}

	fmt.Println(literalSum, count, variableSum)
}
