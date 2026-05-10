package main

import "fmt"

type Runner struct{}

func (r *Runner) RunPiped() string {
	return r.runPiped()
}

func (r *Runner) runPiped() string {
	return "private method"
}

func main() {
	var r Runner
	fmt.Println(r.RunPiped())
}
