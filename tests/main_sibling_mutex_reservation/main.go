package main

import "fmt"

func main() {
	done := make(chan bool)
	go func() {
		value := newMutex()
		fmt.Println(value.Value)
		done <- true
	}()
	<-done
}
