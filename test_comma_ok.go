package main

import "fmt"

func main() {
	var x interface{} = "hello"
	s, ok := x.(string)
	fmt.Println(s, ok)
}
