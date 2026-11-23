package main

import "fmt"

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var x interface{} = func() int { return 5 }
	f := x.(func() int)
	fmt.Println(f())
	fmt.Println(x.(func() int)())
}
