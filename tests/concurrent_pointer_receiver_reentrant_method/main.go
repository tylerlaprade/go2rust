package main

import "fmt"

type state struct {
	value int
}

func (s *state) read() int {
	return s.value
}

func (s *state) run() {
	done := make(chan bool, 1)
	go func() {
		fmt.Println(s.read())
		done <- true
	}()
	fmt.Println(s.read())
	<-done
}

func main() {
	s := &state{value: 7}
	s.run()
}
