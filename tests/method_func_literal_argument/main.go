package main

import "fmt"

type runner struct{}

func (r *runner) Run(f func()) {
	f()
}

func main() {
	go func() {}()

	r := &runner{}
	r.Run(func() {
		fmt.Println("ran")
	})
}
