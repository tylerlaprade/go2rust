package main

import (
	"fmt"

	"example.com/ifaceeq/event"
)

var _ = event.IsMsg

func initConcurrency() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done
}

func main() {
	initConcurrency()
	fmt.Println("compiled")
}
