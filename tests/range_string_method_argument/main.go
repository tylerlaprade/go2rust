package main

import "fmt"

type Encoder struct{}

func (e *Encoder) String(s string) {
	fmt.Println(s)
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	values := []string{"alpha", "beta"}
	var e Encoder
	for _, value := range values {
		e.String(value)
	}
}
