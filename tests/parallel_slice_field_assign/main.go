package main

import "fmt"

type parsed struct {
	kind string
	rest string
}

func split(x string) parsed {
	var p parsed
	p.kind, x = x[:1], x[1:]
	p.rest = x
	return p
}

func main() {
	p := split("abc")
	fmt.Println(p.kind)
	fmt.Println(p.rest)
}
