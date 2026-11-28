package main

import (
	"fmt"

	"example.com/deptypesany/dep"
)

func show(v any) {
	fmt.Printf("%v %T\n", v, v)
}

func main() {
	var a any = dep.New()
	var b any = dep.Label()
	var c any = dep.P()
	var d any = dep.N()
	var e any = dep.Scores()
	show(a)
	show(b)
	show(c)
	show(d)
	show(e)
	fmt.Println(a, b, c, d, e)
	fmt.Println(fmt.Sprint(a), fmt.Sprintf("%d|%v|%s", a, d, b))
	if t, ok := a.(interface{ String() string }); ok {
		fmt.Println("stringer?", t.String())
	} else {
		fmt.Println("tally has no String")
	}
}
