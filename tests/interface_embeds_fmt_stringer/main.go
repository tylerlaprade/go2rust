package main

import "fmt"

type Named interface {
	fmt.Stringer
	Name() string
}

type dog struct{ n string }

func (d dog) String() string { return "dog " + d.n }
func (d dog) Name() string   { return d.n }

func describe(x Named) string { return x.String() + "/" + x.Name() }

func main() {
	var n Named = dog{"rex"}
	fmt.Println(describe(n), n)
}
