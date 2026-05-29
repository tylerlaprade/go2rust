package main

import "fmt"

type Type interface{ tag() string }

type Basic struct{ name string }

func (b *Basic) tag() string { return b.name }

type holder struct{ typ Type }

var Typ = []*Basic{{name: "a"}, {name: "b"}}

func main() {
	var h holder
	h.typ = Typ[1] // *Basic from slice index assigned to a Type field
	var t Type = Typ[0]
	fmt.Println(h.typ.tag(), t.tag())
}
