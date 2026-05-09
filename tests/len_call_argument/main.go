package main

import "fmt"

type Encoder struct{}

func (e *Encoder) Len(x int) {
	fmt.Println(x)
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	values := []string{"alpha", "beta", "gamma"}
	var e Encoder
	e.Len(len(values))
}
