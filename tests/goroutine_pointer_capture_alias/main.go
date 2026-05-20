package main

import "fmt"

type Box struct {
	n int
}

func set(box *Box, done chan bool) {
	box.n = 1
	done <- true
}

func main() {
	b := &Box{}
	done := make(chan bool, 1)

	go func() {
		set(b, done)
	}()

	<-done
	fmt.Println(b.n)
}
