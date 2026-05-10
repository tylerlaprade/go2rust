package main

import "fmt"

type pkg struct {
	name string
}

type reader struct {
	current *pkg
}

func use(p *pkg) {
	fmt.Println(p.name)
}

func (r *reader) run() {
	use(r.current)
}

func main() {
	p := &pkg{name: "alpha"}
	r := reader{current: p}
	r.run()
}
