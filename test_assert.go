package main

import "fmt"

func main() {
	var x interface{} = "hello"
	s := x.(string)
	fmt.Println(s)
}
