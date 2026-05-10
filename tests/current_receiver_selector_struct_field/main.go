package main

import "fmt"

type reader struct {
	path string
}

type carrier struct {
	path string
}

func (c *carrier) printReader() {
	r := reader{path: c.path}
	fmt.Println(r.path)
}

func main() {
	c := carrier{path: "alpha"}
	c.printReader()
}
