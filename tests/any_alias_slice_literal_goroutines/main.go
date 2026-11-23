package main

import "fmt"

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	list := []any{1, "s"}
	fmt.Println(len(list), list[0].(int), list[1].(string))
}
