package main

import "fmt"

type Runner struct {
	name string
}

func (r *Runner) after() {
	_ = r.name
}

func (r *Runner) Run(done chan string) {
	go func() {
		defer r.after()
		done <- r.name
	}()
}

func main() {
	done := make(chan string)
	r := Runner{name: "ok"}
	r.Run(done)
	fmt.Println(<-done)
}
