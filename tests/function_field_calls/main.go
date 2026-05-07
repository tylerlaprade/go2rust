package main

import "fmt"

type Runner struct {
	callback func(string)
}

func printValue(value string) {
	fmt.Println(value)
}

func (r *Runner) Run() {
	r.callback("ok")
}

func main() {
	r := Runner{callback: printValue}
	r.Run()
}
