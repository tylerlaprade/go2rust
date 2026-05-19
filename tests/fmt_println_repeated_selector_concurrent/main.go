package main

import "fmt"

type box struct {
	name  string
	items []int
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	value := box{name: "alpha", items: []int{1, 2}}
	fmt.Println(value.name, len(value.items), value.name)
}
