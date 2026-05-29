package main

import "fmt"

// A named integer type that implements an interface. Unary minus on such a
// value must keep the named type (int64Val), not collapse to the primitive,
// so the result can be boxed as the interface it implements.
type Value interface{ Tag() int64 }

type int64Val int64

func (v int64Val) Tag() int64 { return int64(v) }

func negate(y int64Val) Value {
	z := -y
	return z
}

func main() {
	fmt.Println(negate(5).Tag())
	fmt.Println(negate(-3).Tag())
}
