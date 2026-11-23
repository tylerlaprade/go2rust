package main

import "fmt"

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var x interface{}
	x = map[string]int{"k": 3}
	m, ok := x.(map[string]int)
	fmt.Println(m["k"], ok)
}
