package main

import "fmt"

func run(s Stepper) int {
	s.Step()
	s.Step()
	return s.Value()
}

func main() {
	fmt.Println(run(&Counter{}))
}
