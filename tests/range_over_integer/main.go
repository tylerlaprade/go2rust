package main

import "fmt"

type counter struct {
	n int
}

func (c *counter) Len() int {
	return c.n
}

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

	methodSum := 0
	c := &counter{n: 4}
	for i := range c.Len() {
		methodSum += i
	}

	fmt.Println(literalSum, count, variableSum, methodSum)
}
