package main

import "fmt"

type info struct {
	name string
}

func main() {
	var p *info
	p = &info{name: "ready"}
	fmt.Println(p.name)
}
