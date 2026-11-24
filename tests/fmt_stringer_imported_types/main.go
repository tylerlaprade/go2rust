package main

import (
	"fmt"

	"example.com/fmt_stringer_imported_types/model"
)

type Celsius struct{ Deg int }

func (c Celsius) String() string { return fmt.Sprintf("%dC", c.Deg) }

func makeC() Celsius { return Celsius{3} }

func main() {
	var s fmt.Stringer = makeC()
	fmt.Println(s.String())
	var v any = Celsius{2}
	fmt.Println(v.(fmt.Stringer))
	fmt.Printf("%T\n", v.(fmt.Stringer))
	_, ok := any(model.Label{Text: "x"}).(fmt.Stringer)
	var w any = model.Label{Text: "y"}
	st, ok2 := w.(fmt.Stringer)
	fmt.Println(ok, ok2, st)
}
