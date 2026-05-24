package main

import "fmt"

func main() {
	in := &Inner{Tag: "k", Data: 42}
	out := Outer{Name: "n", Inner: in}
	fmt.Printf("%s %s %v\n", out.Name, out.Inner.Tag, out.Inner.Data)
}
