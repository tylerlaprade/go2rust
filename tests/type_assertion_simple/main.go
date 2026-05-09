package main

import "fmt"

type hasName interface {
	Name() string
}

type hasNameAndString interface {
	hasName
	String() string
}

func assertedAnonymousInterface(v hasNameAndString) bool {
	_, ok := v.(interface{ Name() string })
	return ok
}

func main() {
	var x interface{} = "hello"

	// Type assertion with comma-ok
	if s, ok := x.(string); ok {
		fmt.Println("x is string:", s)
	}

	// Type assertion without comma-ok (would panic if wrong)
	str := x.(string)
	fmt.Println("Asserted string:", str)

	// Failed assertion with comma-ok
	if n, ok := x.(int); ok {
		fmt.Println("x is int:", n)
	} else {
		fmt.Println("x is not an int")
	}
}
