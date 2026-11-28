package main

import (
	"fmt"

	"example.com/helperbindingdep/dep"
)

// count_ is declared beside count, so count takes yet another Rust name.
type count_ struct{ Value int }

type count int

// saved has the dependency's unexported type, which is renamed there.
var saved = dep.New()

func main() {
	var c count = 5
	x := count_{Value: 7}
	d := saved.Double()
	fmt.Println(saved, d, c, x.Value, x)
	fmt.Println(dep.Sum(4))
	h := dep.Hex(171)
	fmt.Println(h)
	hs := []fmt.Stringer{h, dep.Hex(1)}
	for _, s := range hs {
		fmt.Println(s.String())
	}
}
