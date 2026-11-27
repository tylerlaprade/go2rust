package main

import "fmt"

// A fmt.Stringer put back into an interface{} keeps the type of the value
// it holds, so asserting that type finds it (DEV-155).

type Celsius struct{ v int }

func (c Celsius) String() string { return fmt.Sprintf("%dC", c.v) }

type Counter struct{ n int }

func (c *Counter) String() string { return fmt.Sprint("count ", c.n) }

func show(v any) {
	c, ok := v.(Celsius)
	fmt.Println("show", c, ok)
}

func main() {
	var x any = Celsius{5}
	s := x.(fmt.Stringer)
	var y any = s
	c, ok := y.(Celsius)
	fmt.Println(c, ok)

	p := &Counter{1}
	var st fmt.Stringer = p
	q, ok := any(st).(*Counter)
	fmt.Println(ok, q == p)

	show(s)
	list := []any{s, st}
	d, ok := list[0].(Celsius)
	fmt.Println(d, ok)
	_, ok = list[1].(*Counter)
	fmt.Println(ok)
	fmt.Printf("%v %T %T\n", list[0], list[0], list[1])
	var z any = st
	switch v := z.(type) {
	case *Counter:
		fmt.Println("counter", v.n)
	default:
		fmt.Println("other")
	}
	back, ok := z.(fmt.Stringer)
	fmt.Println(back, ok)
	var none fmt.Stringer
	var w any = none
	fmt.Println(w == nil)
	m := map[string]any{"s": s}
	_, ok = m["s"].(Celsius)
	fmt.Println(ok)
}
