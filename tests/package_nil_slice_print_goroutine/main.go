package main

import "fmt"

var global []int
var labels map[string]int

func main() {
	done := make(chan bool)
	go func() {
		fmt.Println(global, labels)
		done <- true
	}()
	<-done
}
