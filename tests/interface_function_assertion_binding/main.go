package main

import "fmt"

func main() {
	var x interface{} = func() int { return 5 }
	f := x.(func() int)
	fmt.Println(f())
	fmt.Println(x.(func() int)())
}
