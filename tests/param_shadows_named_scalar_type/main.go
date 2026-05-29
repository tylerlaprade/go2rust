package main

import "fmt"

// `color` is a named scalar type (a Rust tuple struct). A method parameter also
// named `color` would shadow it, which Rust rejects (E0530); the parameter
// binding must be renamed while the type reference keeps the bare name.
type color uint32

type obj struct{ c color }

func (o *obj) setColor(color color) { o.c = color }

func main() {
	o := &obj{}
	o.setColor(2)
	fmt.Println(uint32(o.c))
}
