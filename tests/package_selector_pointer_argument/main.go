package main

import "example.com/package-selector-pointer-argument/helper"

type reader struct {
	current *helper.Pkg
}

func (r *reader) run() {
	helper.Use(r.current)
}

func main() {
	p := &helper.Pkg{Name: "alpha"}
	r := reader{current: p}
	r.run()
}
